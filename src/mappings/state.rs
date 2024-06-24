//! The states of the controller.
//!
//! States are atomic parts of the controller.

/// The state of a button.
///
/// In the DualSense controller, where all the buttons are digital, there can be only two states,
/// released and pressed.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonState {
    /// The button is released.
    Released,
    /// The button is pressed.
    Pressed,
}

impl ButtonState {
    /// Return `true` if the button is being pressed and `false` otherwise.
    pub fn is_pressed(&self) -> bool {
        *self == Self::Pressed
    }
}

/// A back trigger axis.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Axis(u8);

impl Axis {
    pub(crate) fn new(value: u8) -> Self {
        Axis(value)
    }

    pub(crate) fn value(&self) -> u8 {
        self.0
    }
}

/// Coordinates of the analog stick.
#[derive(Debug, Copy, Clone)]
pub(crate) struct StickCoordinates {
    /// X coordinate.
    pub(crate) x: u8,
    /// Y coordinate.
    pub(crate) y: u8,
}

/// The state of the analog stick.
#[derive(Debug, Copy, Clone)]
pub struct StickState {
    pub(crate) state: ButtonState,
    pub(crate) position: StickCoordinates,
}

impl StickState {
    /// Return the state of the analog stick button (either L3 or R3).
    pub fn button(&self) -> ButtonState {
        self.state
    }

    /// Get the `X` coordinate of the analog stick.
    pub fn x(&self) -> u8 {
        self.position.x
    }

    /// Get the `Y` coordinate of the analog stick.
    pub fn y(&self) -> u8 {
        self.position.y
    }
}

/// Angular velocity of the controller (used for understanding orientation).
#[derive(Debug, Copy, Clone)]
pub struct AngularVelocityState {
    /// X velocity.
    pub(crate) x: i16,
    /// Y velocity.
    pub(crate) y: i16,
    /// Z velocity.
    pub(crate) z: i16,
}

impl AngularVelocityState {
    /// Get the `X` component of the angular velocity.
    pub fn x(&self) -> i16 {
        self.x
    }

    /// Get the `Y` component of the angular velocity.
    pub fn y(&self) -> i16 {
        self.y
    }

    /// Get the `Z` component of the angular velocity.
    pub fn z(&self) -> i16 {
        self.z
    }
}

/// Acceleration of the controller (used for understanding movement).
#[derive(Debug, Copy, Clone)]
pub struct AccelerationState {
    /// X acceleration.
    pub(crate) x: i16,
    /// Y acceleration.
    pub(crate) y: i16,
    /// Z acceleration.
    pub(crate) z: i16,
}

impl AccelerationState {
    /// Get the `X` component of the acceleration.
    pub fn x(&self) -> i16 {
        self.x
    }

    /// Get the `Y` component of the acceleration.
    pub fn y(&self) -> i16 {
        self.y
    }

    /// Get the `Z` component of the acceleration.
    pub fn z(&self) -> i16 {
        self.z
    }
}

// TODO: Maybe change types to be all 32-bits.
/// Temperature of the controller.
#[derive(Debug, Copy, Clone)]
pub enum TemperatureState {
    /// Temperature in Celsius.
    Celsius(i8),
    /// Temperature in Fahrenheit.
    Fahrenheit(u8),
    /// Temperature in Kelvin.
    Kelvin(u16),
}

impl TemperatureState {
    /// Return the temperature as Celsius.
    pub fn as_celcius(&self) -> Self {
        match *self {
            Self::Celsius(t) => Self::Celsius(t),
            Self::Fahrenheit(t) => Self::Celsius(((t - 32) as f32 * 5.0 / 9.0) as i8),
            Self::Kelvin(t) => Self::Celsius((t - 273) as i8),
        }
    }

    /// Return the temperature as Fahrenheit.
    pub fn as_fahrenheit(&self) -> Self {
        match *self {
            Self::Celsius(t) => Self::Fahrenheit((t as f32 * (9.0 / 5.0)) as u8 + 32),
            Self::Fahrenheit(t) => Self::Fahrenheit(t),
            Self::Kelvin(t) => Self::Fahrenheit(((t - 273) as f32 * (9.0 / 5.0)) as u8 + 32),
        }
    }

    /// Return the temperature as Kelvin.
    pub fn as_kelvin(&self) -> Self {
        match *self {
            Self::Celsius(t) => Self::Kelvin((t as f32 + 273.15) as u16),
            Self::Fahrenheit(t) => Self::Kelvin(((t - 32) as f32 * 5.0 / 9.0) as u16 - 273),
            Self::Kelvin(t) => Self::Kelvin(t),
        }
    }
}

/// Data of finger movement in the touchpad.
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FingerData {
    /// Index of the finger.
    ///
    /// Every time a finger touches the touchpad, the index is incremented by one, as to enable
    /// differentiation of each touch.
    pub(crate) index: u8,
    /// If the finger is touching the touchpad.
    pub(crate) is_touching: bool,
    /// X coordinate of the touch.
    pub(crate) x: u16,
    /// Y coordinate of the touch.
    pub(crate) y: u16,
}

/// The State of the touchpad.
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub struct TouchPadState {
    /// If the touchpad is being clicked or not.
    pub(crate) state: ButtonState,
    /// Finger data of up to two fingers.
    pub(crate) finger: [FingerData; 2],
    /// TODO: understand and document.
    pub(crate) timestamp: u8,
}

/// The power state of the controller.
#[derive(Debug, Copy, Clone)]
pub enum PowerState {
    /// Controller is discharging.
    Discharging = 0x00,
    /// Controller is charging.
    Charging = 0x01,
    /// Controller has full battery.
    Complete = 0x02,
    /// Controller is detecting abnormal voltages.
    AbnormalVoltage = 0x0A,
    /// Controller is detecting abnormal temperatures.
    AbnormalTemperature = 0x0B,
    /// Controller cannot charge.
    ChargingError = 0x0F,
}

/// The state of a peripheral device.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PluggedState {
    /// The device is unplugged.
    Unplugged,
    /// The device is plugged.
    Plugged,
}

impl PluggedState {
    /// Return `true` if the peripheral device is plugged and `false` otherwise.
    pub fn is_plugged(&self) -> bool {
        *self == PluggedState::Plugged
    }
}

/// The state of a microphone muted status.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MutedState {
    /// The device is unmuted.
    Unmuted,
    /// The device is muted.
    Muted,
}

impl MutedState {
    /// Return `true` if the microphone is muted and `false` otherwise.
    pub fn is_muted(&self) -> bool {
        *self == MutedState::Muted
    }
}

/// The state of the microphone.
#[derive(Debug, Copy, Clone)]
pub struct MicrophoneState {
    /// Is the microphone plugged.
    pub(crate) state: PluggedState,
    /// Is the microphone muted.
    pub(crate) muted: MutedState,
    /// Is the microphone external.
    pub(crate) external: bool,
}

impl MicrophoneState {
    /// Get the plugged state of the microphone.
    pub fn plugged(&self) -> PluggedState {
        self.state
    }

    /// Return `true` if the microphone is plugged and `false` otherwise.
    pub fn is_plugged(&self) -> bool {
        self.state.is_plugged()
    }

    /// Get the muted state of the microphone.
    pub fn muted(&self) -> MutedState {
        self.muted
    }

    /// Return `true` if the microphone is muted and `false` otherwise.
    pub fn is_muted(&self) -> bool {
        self.muted.is_muted()
    }

    /// Return `true` if the microphone is an external microphone and `false` otherwise.
    pub fn external(&self) -> bool {
        self.external
    }
}

/// The state of the USB.
#[derive(Debug, Copy, Clone)]
pub struct USBState {
    /// Is the USB data capable.
    pub(crate) data: PluggedState,
    /// Is the USB power capable.
    pub(crate) power: PluggedState,
}

impl USBState {
    /// Get the state of the data capabilities.
    pub fn data(&self) -> PluggedState {
        self.data
    }

    /// Get the state of the power capabilities
    pub fn power(&self) -> PluggedState {
        self.power
    }
}

/// The current status of the back trigger effect.
///
/// This changes depending on the type of effect in action, you can read more about it in
/// [`BackTriggerEffect`]
///
/// [`BackTriggerEffect`]: enum@crate::mappings::BackTriggerEffect
#[derive(Debug, Copy, Clone)]
pub enum BackTriggerStatus {
    /// No feedback load is being applied.
    FeedbackNoLoad,
    /// Feedback load is being applied.
    FeedbackLoadApplied,
    /// Weapon is ready to be fired.
    WeaponReady,
    /// Weapon is firing.
    WeaponFiring,
    /// Weapon has already fired.
    WeaponFired,
    /// Trigger is not vibrating.
    VibrationNotVibrating,
    /// Trigger is vibrating.
    VibrationIsVibrating,
}

/// The applied effect to the back trigger.
///
/// Different effects apply different resistance curves and vibration patterns.
#[derive(Debug, Copy, Clone)]
pub enum BackTriggerEffect {
    /// Turn the trigger effect off and return the trigger stop to the neutral position.
    ///
    /// The [`BackTriggerStatus`] will always return `FeedbackNoLoad` during it.
    ///
    /// [`BackTriggerStatus`]: enum@crate::mappings::BackTriggerStatus
    Off,
    /// Trigger will resist movement beyond the start position.
    ///
    /// The [`BackTriggerStatus`] will return `FeedbackNoLoad` before the effect and
    /// `FeedbackLoadApplied` after the effect.
    ///
    /// [`BackTriggerStatus`]: enum@crate::mappings::BackTriggerStatus
    Feedback,
    /// Trigger will resist movement beyond the start position until the end position.
    ///
    /// The [`BackTriggerStatus`] will return `WeaponReady` before the effect, `WeaponFiring`
    /// during the effect and `WeaponFired` after the effect.
    ///
    /// [`BackTriggerStatus`]: enum@crate::mappings::BackTriggerStatus
    Weapon,
    /// Trigger will vibrate with the input amplitude and frequency beyond the start position.
    ///
    /// The [`BackTriggerStatus`] will return `VibrationNotVibrating` before the effect and
    /// `VibrationIsVibrating` during the effect.
    ///
    /// [`BackTriggerStatus`]: enum@crate::mappings::BackTriggerStatus
    Vibration,
}

// TODO: Understand better what stop location means.
#[allow(missing_docs, unused)]
#[derive(Debug, Copy, Clone)]
pub struct BackTriggerStop(pub u8);

/// A state of the back trigger.
///
/// Different from the [`front trigger`], a back trigger is composed of a more complex state. As
/// such, more information is needed to fully represent it.
///
/// [`front trigger`]: enum@crate::mappings::ButtonState
#[derive(Debug, Copy, Clone)]
pub struct BackTriggerState {
    /// Binary state of the back trigger.
    ///
    /// The state is `Released` when the [`axis`] is zero, and `Pressed` otherwise.
    ///
    /// [`axis`]: Self::axis
    pub(crate) state: ButtonState,
    /// Axis of the back trigger.
    pub(crate) axis: Axis,
    /// Effect applied to the back trigger.
    pub(crate) effect: BackTriggerEffect,
    /// Status of the back trigger.
    ///
    /// The status relates to the effect.
    pub(crate) status: BackTriggerStatus,
    // TODO: Understand it.
    #[allow(unused)]
    pub(crate) stop: BackTriggerStop,
}

impl BackTriggerState {
    /// Get the button state of the back trigger.
    ///
    /// Will return [`ButtonState::Pressed`] when [`axis`] is above `0`, and
    /// [`ButtonState::Released`] otherwise.
    ///
    /// [`axis`]: fn@Self::axis
    pub fn button(&self) -> ButtonState {
        self.state
    }

    /// Get the current axis of the back trigger.
    ///
    /// An axis is a value ranging from `0` to `255`, representing how far pressed a back trigger
    /// is. `0` indicates that the trigger is 0% pressed, while `255` indicates that the trigger is
    /// 100% pressed.
    ///
    /// A common problem that can happen with time is the degradation of the back triggers due to
    /// persistent pressing. When that happens, the trigger can no longer achieve a value of `255`,
    /// stopping at a lower amount.
    pub fn axis(&self) -> u8 {
        self.axis.value()
    }

    /// Get the current effect of the back trigger.
    ///
    /// The DualSense controller has support for different pre-defined effects (they can change
    /// with firmware updates, but their names and what they do are not publicly shared). For a
    /// list with the officially supported effects you can see [`BackTriggerEffect`].
    ///
    /// [`BackTriggerEffect`]: enum@crate::mappings::BackTriggerEffect
    pub fn effect(&self) -> BackTriggerEffect {
        self.effect
    }

    /// Get the current status of the back trigger.
    ///
    ///
    /// The back trigger status indicates the state of the back trigger in a manner relevant to the
    /// effect. While the [`axis`] can show absolute positioning, the status will show what is the
    /// effect being applied, and if it is being currently applied or not.
    ///
    /// [`axis`]: fn@Self::axis
    pub fn status(&self) -> BackTriggerStatus {
        self.status
    }
}

/// Direction of the directional pad.
///
/// The directional pad is defined as the four arrow buttons in the left of the face of the
/// controller. At most two neighboring buttons can be pressed at once. Because of this, their
/// state is usually represented as directions in a compass.
#[derive(Debug, Copy, Clone)]
pub enum DPadDirection {
    /// Up arrow pressed.
    North,
    /// Up and left arrow pressed.
    NorthEast,
    /// Left arrow pressed.
    East,
    /// Left and down arrow pressed.
    SouthEast,
    /// Down arrow pressed.
    South,
    /// Down and left arrow pressed.
    SouthWest,
    /// Left arrow pressed.
    West,
    /// Left and up arrow pressed.
    NorthWest,
    /// No arrow pressed.
    None,
}

impl From<u8> for ButtonState {
    fn from(value: u8) -> Self {
        assert!(value < 2, "Out of range for ButtonState");

        match value {
            0 => ButtonState::Released,
            1 => ButtonState::Pressed,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for PluggedState {
    fn from(value: u8) -> Self {
        assert!(value < 2, "Out of range for PluggedState");

        match value {
            0 => PluggedState::Unplugged,
            1 => PluggedState::Plugged,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for MutedState {
    fn from(value: u8) -> Self {
        assert!(value < 2, "Out of range for MutedState");

        match value {
            0 => MutedState::Unmuted,
            1 => MutedState::Muted,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for PowerState {
    fn from(value: u8) -> Self {
        assert!(value <= 0x0F, "Out of range for PowerState");

        match value {
            0x00 => PowerState::Discharging,
            0x01 => PowerState::Charging,
            0x02 => PowerState::Complete,
            0x0A => PowerState::AbnormalVoltage,
            0x0B => PowerState::AbnormalTemperature,
            0x0F => PowerState::ChargingError,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for DPadDirection {
    fn from(value: u8) -> Self {
        assert!(value < 9, "Out of range for DPadDirection");

        match value {
            0 => DPadDirection::North,
            1 => DPadDirection::NorthEast,
            2 => DPadDirection::East,
            3 => DPadDirection::SouthEast,
            4 => DPadDirection::South,
            5 => DPadDirection::SouthWest,
            6 => DPadDirection::West,
            7 => DPadDirection::NorthWest,
            8 => DPadDirection::None,
            _ => unreachable!(),
        }
    }
}
impl From<u8> for BackTriggerEffect {
    fn from(value: u8) -> Self {
        assert!(value < 16, "Out of range for BackTriggerEffect");

        match value {
            0 => BackTriggerEffect::Off,
            1 => BackTriggerEffect::Feedback,
            2 => BackTriggerEffect::Weapon,
            3 => BackTriggerEffect::Vibration,
            _ => unreachable!(),
        }
    }
}

impl From<(u8, BackTriggerEffect)> for BackTriggerStatus {
    fn from(value: (u8, BackTriggerEffect)) -> Self {
        let (value, effect) = value;

        assert!(value < 3, "Out of range for BackTriggerStatus");

        match effect {
            BackTriggerEffect::Off => BackTriggerStatus::FeedbackNoLoad,
            BackTriggerEffect::Feedback => match value {
                0 => BackTriggerStatus::FeedbackNoLoad,
                1 => BackTriggerStatus::FeedbackLoadApplied,
                _ => unreachable!(),
            },
            BackTriggerEffect::Weapon => match value {
                0 => BackTriggerStatus::WeaponReady,
                1 => BackTriggerStatus::WeaponFiring,
                2 => BackTriggerStatus::WeaponFired,
                _ => unreachable!(),
            },
            BackTriggerEffect::Vibration => match value {
                0 => BackTriggerStatus::VibrationNotVibrating,
                1 => BackTriggerStatus::VibrationIsVibrating,
                _ => unreachable!(),
            },
        }
    }
}
