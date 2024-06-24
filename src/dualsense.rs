//! The DualSense controller module.
//!
//! This module provides the core functionality for the _duplosentido_ crate.

use crate::hidapi::{self, DeviceWrapper, ProductID, RawInputReportUSB, VendorID};
use crate::mappings::group::{
    ActionButtonGroup, BackTriggerGroup, FrontTriggerGroup, MenuGroup, PluggedGroup, PowerGroup,
    StickGroup,
};
use crate::mappings::{
    AccelerationState, AngularVelocityState, Axis, BackTriggerEffect, BackTriggerState,
    BackTriggerStatus, BackTriggerStop, ButtonState, DPadDirection, FingerData, MicrophoneState,
    MutedState, PluggedState, PowerState, StickCoordinates, StickState, TemperatureState,
    TouchPadState, USBState,
};

use std::cell::Cell;
use thiserror::Error;

/// A specialized Result type for DualSense controller interactions.
///
/// This type is used across [`duplosentido`] for any operation which may produce an error. This
/// typedef is generally used to avoid writing out [`duplosentido::Error`] directly and is
/// otherwise a direct mapping to [`std::result::Result`].
///
/// [`duplosentido`]: crate
/// [`duplosentido::Error`]: enum@crate::Error
/// [`std::result::Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The error type for operations with a DualSense controller.
#[derive(Error, Debug)]
pub enum Error {
    /// A bind error.
    ///
    /// This error can happen when trying to bind with a controller.
    #[error("Could not bind with a controller")]
    Bind,
    /// A mode error.
    ///
    /// This error can happen when trying to change the update mode (_blocking_ or _non-blocking_)
    /// of the controller.
    #[error("Could not change mode")]
    Mode,
    /// An update error.
    ///
    /// This error can happen when trying to update the controller state.
    #[error("Could not update controller state")]
    Update,
    /// An exit error.
    ///
    /// This error can happen when trying to finish using the controller (usually when dropping
    /// it).
    #[error("Could not properly clean up at controller exit")]
    Exit,
}

impl From<hidapi::Error> for Error {
    fn from(value: hidapi::Error) -> Self {
        // This will *not* hold true anymore if these errors are used in different places
        // that do not uphold this pattern. So always check before.
        match value {
            hidapi::Error::Open => Error::Bind,
            hidapi::Error::Mode => Error::Mode,
            hidapi::Error::Read => Error::Update,
            hidapi::Error::Exit => Error::Exit,
        }
    }
}

/// The mode to use when updating the controller state.
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    /// Blocking mode.
    ///
    /// In this mode, every call to [`update`] will block until either data is read or
    /// an error occurs.
    ///
    /// [`update`]: fn@crate::DualSense::update
    Blocking = 0,

    /// Non-blocking mode.
    ///
    /// In this mode, every call to [`update`] will immediately return independently if
    /// data was read or not.
    ///
    /// [`update`]: fn@crate::DualSense::update
    NonBlocking = 1,
}

/// A bind to a DualSense controller.
pub struct DualSense {
    controller: hidapi::DeviceWrapper,
    // Use `RefCell` to avoid the need for the user to declare the controller as `mut`, since it
    // isn't intuitive for it to be `mut` in this case, as `mut` should imply that we are mutating
    // the controller itself.
    state: Cell<DualSenseState>,
    mode: Cell<Mode>,
}

impl DualSense {
    /// Try connecting with a DualSense controller.
    ///
    /// In case multiple controllers are found, only the first one listed will be binded to.
    ///
    /// This method can fail either if a DualSense controller is not found or if it wasn't possible
    /// to bind with the controller.
    pub fn bind() -> Result<Self> {
        const VENDOR_ID: VendorID = VendorID::new(0x054C);
        const PRODUCT_ID: ProductID = ProductID::new(0x0CE6);

        match DeviceWrapper::open(VENDOR_ID, PRODUCT_ID) {
            Ok(controller) => {
                // Set the mode to blocking. Since the default DualSense poll rate is 250hz, every
                // 4ms we receive a new reading.
                controller.set_mode(Mode::Blocking)?;

                // TODO: Find the best approach to handle uninitialized (not `update`d) controller
                // state.
                let state = DualSenseState::from(RawInputReportUSB::new([0_u8; 64])).into();
                let mode = Mode::Blocking.into();

                Ok(DualSense {
                    controller,
                    state,
                    mode,
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Update the current controller state.
    ///
    /// If the mode is set to _blocking_, the call to [`update`] will block until the controller
    /// sends its new state. In the worst case, this can take up to 4ms.
    ///
    /// To avoid unnecessary delays, you should call [`update`] only at the start of your
    /// operations to update the controller state. After that, you can call [`state`] and use the
    /// state of the controller without the need of updating for every state check.
    ///
    /// # Blocking mode
    /// In _blocking_ mode, if the method returns `Ok`, it is *guaranteed* to have updated the
    /// state. As such, you can safely ignore the number of returned bytes.
    ///
    /// # Non-blocking mode
    /// In _non-blocking_ mode there is no guarantee that the call to [`update`] read any bytes.
    /// For that reason, you should check the number of bytes read by the method.
    ///
    /// [`update`]: fn@crate::DualSense::update
    /// [`state`]: fn@crate::DualSense::state
    pub fn update(&self) -> Result<usize> {
        // 64 bytes is the maximum size of a packet in wired mode, so we can use a known size
        // slice. For Bluetooth mode, it seems that reports can get as big as 546 bytes (!), so if
        // we plan on supporting it in the future, we may need to change the slice to a `Vec`.
        let mut buffer = [0_u8; 64];
        let bytes = self.controller.read(&mut buffer)?;
        if bytes == 0 {
            return Ok(bytes);
        }

        // Guard against other types of reports, see
        // https://controllers.fandom.com/wiki/Sony_DualSense#USB for more information.
        assert!(
            bytes == 64,
            "Only one type of report is currently implemented"
        );
        match buffer[0] {
            0x01 => (),
            _ => unimplemented!(),
        }

        let state = DualSenseState::from(RawInputReportUSB::new(buffer));
        self.state.replace(state);

        Ok(bytes)
    }

    /// Set mode to be either _blocking_ or _non-blocking_.
    pub fn set_mode(&self, mode: Mode) -> Result<()> {
        // TODO: Understand why it can fail.
        self.controller.set_mode(mode)?;
        self.mode.replace(mode);

        Ok(())
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.mode.get()
    }

    /// Get the current controller state.
    ///
    /// If [`update`] was not called at least once, the returned state is not representative of the
    /// controller *actual* state. As such, this method should be called *only* after at least one
    /// update from the controller.
    ///
    /// [`update`]: fn@crate::DualSense::update
    pub fn state(&self) -> DualSenseState {
        self.state.get()
    }
}

impl Drop for DualSense {
    fn drop(&mut self) {
        // TODO: Unwrapping is not the best choice, maybe do something else?
        hidapi::exit().unwrap();
    }
}

/// The state of a DualSense controller.
///
/// This state has all the common buttons and readings from the controller. There is more
/// information that could be read from the controller. But, in such cases, you should
/// call...(TODO: continue with other reports maybe)
#[derive(Debug, Copy, Clone)]
pub struct DualSenseState {
    sticks: StickGroup,
    directional_pad: DPadDirection,
    action_buttons: ActionButtonGroup,
    menus: MenuGroup,
    // TODO: Find a good and simple way to expose the TouchPad.
    #[allow(unused)]
    touchpad: TouchPadState,
    front_triggers: FrontTriggerGroup,
    back_triggers: BackTriggerGroup,
    angular_velocity: AngularVelocityState,
    acceleration: AccelerationState,
    plugged: PluggedGroup,
    temperature: TemperatureState,
    power: PowerGroup,
}

impl DualSenseState {
    /// Get the left analog stick state.
    pub fn left_stick(&self) -> StickState {
        self.sticks.left
    }

    /// Get the right analog stick state.
    pub fn right_stick(&self) -> StickState {
        self.sticks.right
    }

    /// Get the `Square` button state.
    pub fn square(&self) -> ButtonState {
        self.action_buttons.square
    }

    /// Get the `Triangle` button state.
    pub fn triangle(&self) -> ButtonState {
        self.action_buttons.triangle
    }

    /// Get the `Circle` button state.
    pub fn circle(&self) -> ButtonState {
        self.action_buttons.circle
    }

    /// Get the `Cross` button state.
    pub fn cross(&self) -> ButtonState {
        self.action_buttons.cross
    }

    /// Get the directional pad state.
    pub fn dpad(&self) -> DPadDirection {
        self.directional_pad
    }

    /// Get the `Create` button state.
    pub fn create_menu(&self) -> ButtonState {
        self.menus.create
    }

    /// Get the `Options` button state.
    pub fn options_menu(&self) -> ButtonState {
        self.menus.options
    }

    /// Get the `Home (PS)` button state.
    pub fn home_menu(&self) -> ButtonState {
        self.menus.home
    }

    /// Get the `Mute` button state.
    pub fn mute_menu(&self) -> ButtonState {
        self.menus.mute
    }

    ///  Get the `L1` button state.
    pub fn l1(&self) -> ButtonState {
        self.front_triggers.l1
    }

    ///  Get the `R1` button state.
    pub fn r1(&self) -> ButtonState {
        self.front_triggers.r1
    }

    /// Get the `L2` button state.
    pub fn l2(&self) -> BackTriggerState {
        self.back_triggers.l2
    }

    /// Get the `R2` button state.
    pub fn r2(&self) -> BackTriggerState {
        self.back_triggers.r2
    }

    /// Get the angular velocity of the controller.
    pub fn gyroscope(&self) -> AngularVelocityState {
        self.angular_velocity
    }

    /// Get the acceleration of the controller.
    pub fn acceleration(&self) -> AccelerationState {
        self.acceleration
    }

    /// Get the state of the headphone.
    pub fn headphone(&self) -> PluggedState {
        self.plugged.headphone
    }

    /// Get the state of the microphone.
    pub fn microphone(&self) -> MicrophoneState {
        self.plugged.microphone
    }

    /// Get the state of the USB.
    pub fn usb(&self) -> USBState {
        self.plugged.usb
    }

    /// Get the temperature of the controller.
    pub fn temperature(&self) -> TemperatureState {
        self.temperature
    }

    /// Get the state of the controller battery.
    pub fn battery_status(&self) -> PowerState {
        self.power.state
    }

    /// Get the percentage of the controller battery.
    pub fn battery_percent(&self) -> u8 {
        self.power.percent
    }
}

impl From<RawInputReportUSB> for DualSenseState {
    fn from(value: RawInputReportUSB) -> Self {
        let value = value.as_array();

        assert!(
            value[0] == 1 || value[0] == 0,
            "Report must be either of type 1 or empty"
        );

        let mask_shift = |byte: u8, mask: u8| (byte & mask) >> mask.trailing_zeros();

        let sticks = {
            const L3_MASK: u8 = 0b0100_0000;
            const R3_MASK: u8 = 0b1000_0000;

            let state = ButtonState::from(mask_shift(value[9], L3_MASK));
            let position = StickCoordinates {
                x: value[1],
                y: value[2],
            };
            let left = StickState { state, position };

            let state = ButtonState::from(mask_shift(value[9], R3_MASK));
            let position = StickCoordinates {
                x: value[3],
                y: value[4],
            };
            let right = StickState { state, position };

            StickGroup { left, right }
        };

        let directional_pad = {
            const DPAD_MASK: u8 = 0b0000_1111;

            DPadDirection::from(mask_shift(value[8], DPAD_MASK))
        };

        let action_buttons = {
            const SQUARE_MASK: u8 = 0b0001_0000;
            const CROSS_MASK: u8 = 0b0010_0000;
            const CIRCLE_MASK: u8 = 0b0100_0000;
            const TRIANGLE_MASK: u8 = 0b1000_0000;

            let byte = value[8];

            let square = ButtonState::from(mask_shift(byte, SQUARE_MASK));
            let cross = ButtonState::from(mask_shift(byte, CROSS_MASK));
            let circle = ButtonState::from(mask_shift(byte, CIRCLE_MASK));
            let triangle = ButtonState::from(mask_shift(byte, TRIANGLE_MASK));

            ActionButtonGroup {
                square,
                cross,
                circle,
                triangle,
            }
        };

        let menus = {
            const CREATE_MASK: u8 = 0b0001_0000;
            const OPTIONS_MASK: u8 = 0b0010_0000;
            const HOME_MASK: u8 = 0b0000_0001;
            const MUTE_MASK: u8 = 0b0000_0100;

            let create = ButtonState::from(mask_shift(value[9], CREATE_MASK));
            let options = ButtonState::from(mask_shift(value[9], OPTIONS_MASK));
            let home = ButtonState::from(mask_shift(value[10], HOME_MASK));
            let mute = ButtonState::from(mask_shift(value[10], MUTE_MASK));

            MenuGroup {
                create,
                options,
                home,
                mute,
            }
        };

        let touchpad = {
            const TOUCHPAD_MASK: u8 = 0b0000_0010;

            let state = ButtonState::from(mask_shift(value[10], TOUCHPAD_MASK));
            let finger = {
                const INDEX_MASK: u8 = 0b0111_1111;
                const TOUCHING_MASK: u8 = 0b1000_0000;
                const X_MASK: u8 = 0b0000_1111;
                const Y_MASK: u8 = 0b1111_0000;

                let index = value[33] & INDEX_MASK;
                let touching = mask_shift(value[33], TOUCHING_MASK) == 0;
                let x = u16::from_ne_bytes([value[34], value[35] & X_MASK]);
                let y = u16::from_ne_bytes([value[35] & Y_MASK, value[36]]);
                let one = FingerData {
                    index,
                    is_touching: touching,
                    x,
                    y,
                };

                let index = value[37] & INDEX_MASK;
                let touching = mask_shift(value[37], TOUCHING_MASK) == 0;
                let x = u16::from_ne_bytes([value[38], value[39] & X_MASK]);
                let y = u16::from_ne_bytes([value[39] & Y_MASK, value[40]]);
                let two = FingerData {
                    index,
                    is_touching: touching,
                    x,
                    y,
                };

                [one, two]
            };
            let timestamp = value[41];

            TouchPadState {
                state,
                finger,
                timestamp,
            }
        };

        let front_triggers = {
            const L1_MASK: u8 = 0b0000_0001;
            const R1_MASK: u8 = 0b0000_0010;

            let byte = value[9];

            let l1 = ButtonState::from(mask_shift(byte, L1_MASK));
            let r1 = ButtonState::from(mask_shift(byte, R1_MASK));

            FrontTriggerGroup { l1, r1 }
        };

        let back_triggers = {
            const L2_MASK: u8 = 0b0000_0100;
            const L2_EFFECT_MASK: u8 = 0b1111_0000;
            const L2_STATUS_MASK: u8 = 0b1111_0000;
            const L2_STOP_MASK: u8 = 0b1111_0000;
            const R2_MASK: u8 = 0b0000_1000;
            const R2_EFFECT_MASK: u8 = 0b0000_1111;
            const R2_STATUS_MASK: u8 = 0b1111_0000;
            const R2_STOP_MASK: u8 = 0b1111_0000;

            let state = ButtonState::from(mask_shift(value[9], L2_MASK));
            let axis = Axis::new(value[5]);
            let effect = BackTriggerEffect::from(mask_shift(value[48], L2_EFFECT_MASK));
            let status = BackTriggerStatus::from((mask_shift(value[43], L2_STATUS_MASK), effect));
            let stop = BackTriggerStop(mask_shift(value[43], L2_STOP_MASK));
            let l2 = BackTriggerState {
                state,
                axis,
                effect,
                status,
                stop,
            };

            let state = ButtonState::from(mask_shift(value[9], R2_MASK));
            let axis = Axis::new(value[6]);
            let effect = BackTriggerEffect::from(mask_shift(value[48], R2_EFFECT_MASK));
            let status = BackTriggerStatus::from((mask_shift(value[42], R2_STATUS_MASK), effect));
            let stop = BackTriggerStop(mask_shift(value[42], R2_STOP_MASK));
            let r2 = BackTriggerState {
                state,
                axis,
                effect,
                status,
                stop,
            };

            BackTriggerGroup { l2, r2 }
        };

        let angular_velocity = AngularVelocityState {
            x: i16::from_ne_bytes(value[16..=17].try_into().unwrap()),
            y: i16::from_ne_bytes(value[20..=21].try_into().unwrap()),
            z: i16::from_ne_bytes(value[18..=19].try_into().unwrap()),
        };

        let acceleration = AccelerationState {
            x: i16::from_ne_bytes(value[22..=23].try_into().unwrap()),
            y: i16::from_ne_bytes(value[24..=25].try_into().unwrap()),
            z: i16::from_ne_bytes(value[26..=27].try_into().unwrap()),
        };

        let plugged = {
            const HEADPHONE_MASK: u8 = 0b0000_0001;
            const HAPTIC_MASK: u8 = 0b0000_0010;

            let headphone = PluggedState::from(mask_shift(value[54], HEADPHONE_MASK));
            let microphone = {
                const MICROPHONE_MASK: u8 = 0b0000_0010;
                const MUTED_MASK: u8 = 0b0000_0100;
                const EXTERNAL_MASK: u8 = 0b0000_0001;

                let state = PluggedState::from(mask_shift(value[54], MICROPHONE_MASK));
                let muted = MutedState::from(mask_shift(value[54], MUTED_MASK));
                let external = mask_shift(value[55], EXTERNAL_MASK) != 0;

                MicrophoneState {
                    state,
                    muted,
                    external,
                }
            };
            let usb = {
                const DATA_MASK: u8 = 0b0000_1000;
                const POWER_MASK: u8 = 0b0001_0000;

                let data = PluggedState::from(mask_shift(value[54], DATA_MASK));
                let power = PluggedState::from(mask_shift(value[54], POWER_MASK));

                USBState { data, power }
            };
            let haptic_low_pass_filter = PluggedState::from(mask_shift(value[55], HAPTIC_MASK));

            PluggedGroup {
                headphone,
                microphone,
                usb,
                haptic_low_pass_filter,
            }
        };

        let temperature = TemperatureState::Celsius(i8::from_ne_bytes([value[32]]));

        let power = {
            const STATE_MASK: u8 = 0b1111_0000;
            const PERCENT_MASK: u8 = 0b0000_1111;

            let state = PowerState::from(mask_shift(value[53], STATE_MASK));
            let percent = mask_shift(value[53], PERCENT_MASK);
            PowerGroup { state, percent }
        };

        DualSenseState {
            sticks,
            directional_pad,
            action_buttons,
            menus,
            touchpad,
            front_triggers,
            back_triggers,
            angular_velocity,
            acceleration,
            plugged,
            temperature,
            power,
        }
    }
}
