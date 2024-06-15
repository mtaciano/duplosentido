//! The DualSense controller module.

use crate::{
    hidapi::{HidDevice, HidError, Mode, ProductID, VendorID},
    state::*,
};
use std::cell::RefCell;

/// The DualSense controller.
///
/// # Usage
/// You should initialize the controller with [`DualSense::new()`]. The initialization can fail if
/// a controller is not found. After initialization, you can update the controller with
/// [`DualSense::read()`]. To access any information you should call the corresponding function.
///
/// # Caveats
/// Since this crate is in early development, there are some features missing. They are listed
/// below:
/// - Currently, the only supported backend is `hidapi-hidraw`.
/// - The only support is for connection via USB, Bluetooth is not yet supported.
/// - Features are missing (gyro, touchpad, setting trigger effects, LEDs, vibrations, etc).
pub struct DualSense {
    device: HidDevice,
    // Use `RefCell` to avoid having the need for the user to declare the controller as `mut`.
    packet: RefCell<[u8; 64]>,
}

impl DualSense {
    /// Try connecting to a new DualSense controller.
    ///
    /// This method can fail if a DualSense controller could not be found.
    pub fn new() -> Result<Self, HidError> {
        const VENDOR_ID: VendorID = VendorID(0x054C);
        const PRODUCT_ID: ProductID = ProductID(0x0CE6);

        match HidDevice::open(&VENDOR_ID, &PRODUCT_ID) {
            Ok(device) => {
                // Set the mode to blocking, since the default DualSense poll rate is 250hz.
                // This means that every 4ms we receive a new reading.
                // TODO: Implement a non-blocking call in the future.
                device.set_mode(Mode::Blocking)?;

                Ok(DualSense {
                    device,
                    packet: [0_u8; 64].into(),
                })
            }
            Err(e) => Err(e),
        }
    }

    /// Read the current controller state.
    ///
    /// The call to [`read`](Self::read) will block until a packet is received. In the worst case,
    /// this can take up to 4ms.
    ///
    /// To avoid unnecessary delays, you should call [`read`](Self::read) only at the start of your
    /// operations to update the controller state. After that, call the desired functions without
    /// the read.
    pub fn read(&self) -> Result<&Self, HidError> {
        // TODO: Handle non-blocking calls and connection via Bluetooth.
        let bytes = self.device.read(&mut *self.packet.borrow_mut())?;
        assert!(
            bytes == 64,
            "Only connection via USB is currently supported"
        );

        Ok(self)
    }

    /// Get the directional buttons current state.
    ///
    /// This function returns the struct [`DirectionalState`]. This struct
    /// represents the current (i.e. last read) state of the directional buttons (arrows).
    pub fn directional_buttons(&self) -> DirectionalState {
        // Byte 8 (first half) - Directional Buttons
        // Neutral: 0x8, N: 0x0, NE: 0x1, E: 0x2, SE: 0x3, S: 0x4, SW: 0x5, W: 0x6, NW: 0x7
        const DIRECTIONAL_MASK: u8 = 0b0000_1111;
        let byte = self.packet.borrow()[8];

        let (top, right, bottom, left) = match byte & DIRECTIONAL_MASK {
            0x0 => (
                ButtonState::Pressed,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
            ),
            0x1 => (
                ButtonState::Pressed,
                ButtonState::Pressed,
                ButtonState::Released,
                ButtonState::Released,
            ),
            0x2 => (
                ButtonState::Released,
                ButtonState::Pressed,
                ButtonState::Released,
                ButtonState::Released,
            ),
            0x3 => (
                ButtonState::Released,
                ButtonState::Pressed,
                ButtonState::Pressed,
                ButtonState::Released,
            ),
            0x4 => (
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Pressed,
                ButtonState::Released,
            ),
            0x5 => (
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Pressed,
                ButtonState::Pressed,
            ),
            0x6 => (
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Pressed,
            ),
            0x7 => (
                ButtonState::Pressed,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Pressed,
            ),
            0x8 => (
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
                ButtonState::Released,
            ),
            _ => {
                unreachable!()
            }
        };

        DirectionalState {
            bottom,
            left,
            top,
            right,
        }
    }

    /// Get the action buttons current state.
    ///
    /// This function returns the struct [`ActionState`]. This struct represents the current
    /// (i.e. last read) state of the action buttons (square, triangle, circle, cross).
    pub fn action_buttons(&self) -> ActionState {
        // From left to right.
        // Byte 8 (second half) - Action Buttons
        // Byte 8 (Bit 4) - Square Button
        // Byte 8 (Bit 5) - Cross Button
        // Byte 8 (Bit 6) - Circle Button
        // Byte 8 (Bit 7) - Triangle Button
        const SQUARE_MASK: u8 = 0b0001_0000;
        const CROSS_MASK: u8 = 0b0010_0000;
        const CIRCLE_MASK: u8 = 0b0100_0000;
        const TRIANGLE_MASK: u8 = 0b1000_0000;

        let byte = self.packet.borrow()[8];

        let square = ButtonState::from((byte & SQUARE_MASK) >> SQUARE_MASK.trailing_zeros());
        let cross = ButtonState::from((byte & CROSS_MASK) >> CROSS_MASK.trailing_zeros());
        let circle = ButtonState::from((byte & CIRCLE_MASK) >> CIRCLE_MASK.trailing_zeros());
        let triangle = ButtonState::from((byte & TRIANGLE_MASK) >> TRIANGLE_MASK.trailing_zeros());

        ActionState {
            cross,
            square,
            triangle,
            circle,
        }
    }

    /// Get the triggers current state.
    ///
    /// This function returns the struct [`TriggerState`]. This struct represents the current
    /// (i.e. last read) state of the triggers.
    pub fn triggers(&self) -> TriggerState {
        // from right to left
        // byte 5, bit 0: Generic Desktop / Rx - 8 bits - L2 trigger axis
        //   neutral: 0x00, pressed: 0xff
        // byte 6, bit 0: Generic Desktop / Ry - 8 bits - R2 trigger axis
        //   neutral: 0x00, pressed: 0xff
        // byte 9, bit 0: Button / 0x05 - 1 bit - L1 button
        // byte 9, bit 1: Button / 0x06 - 1 bit - R1 button
        // byte 9, bit 2: Button / 0x07 - 1 bit - L2 button
        // byte 9, bit 3: Button / 0x08 - 1 bit - R2 button
        const L1_MASK: u8 = 0b0000_0001;
        const R1_MASK: u8 = 0b0000_0010;
        const L2_MASK: u8 = 0b0000_0100;
        const R2_MASK: u8 = 0b0000_1000;

        let byte = self.packet.borrow()[9];

        let l1 = {
            let state = ButtonState::from((byte & L1_MASK) >> L1_MASK.trailing_zeros());
            Trigger { state, axis: None }
        };
        let r1 = {
            let state = ButtonState::from((byte & R1_MASK) >> R1_MASK.trailing_zeros());
            Trigger { state, axis: None }
        };
        let l2 = {
            let state = ButtonState::from((byte & L2_MASK) >> L2_MASK.trailing_zeros());
            let axis = self.packet.borrow()[5];
            Trigger {
                state,
                axis: Some(Axis(axis)),
            }
        };
        let r2 = {
            let state = ButtonState::from((byte & R2_MASK) >> R2_MASK.trailing_zeros());
            let axis = self.packet.borrow()[6];
            Trigger {
                state,
                axis: Some(Axis(axis)),
            }
        };

        TriggerState { l1, r1, l2, r2 }
    }

    /// Get the analog sticks current state.
    ///
    /// This function returns the struct [`AnalogState`]. This struct represents the current
    /// (i.e. last read) state of the analog sticks.
    pub fn analogs(&self) -> AnalogState {
        // byte 1, bit 0: Generic Desktop / X - 8 bits - left stick X axis
        //   left: 0x00, right: 0xff, neutral: ~0x80
        // byte 2, bit 0: Generic Desktop / Y - 8 bits - left stick Y axis
        //   up: 0x00, down: 0xff, neutral: ~0x80
        // byte 3, bit 0: Generic Desktop / Z - 8 bits - right stick X axis
        //   left: 0x00, right: 0xff, neutral: ~0x80
        // byte 4, bit 0: Generic Desktop / Rz - 8 bits - right stick Y axis
        //   up: 0x00, down: 0xff, neutral: ~0x80
        // byte 9, bit 6: Button / 0x0b - 1 bit - L3 button
        // byte 9, bit 7: Button / 0x0c - 1 bit - R3 button
        const L3_MASK: u8 = 0b0100_0000;
        const R3_MASK: u8 = 0b1000_0000;

        let byte = self.packet.borrow()[9];

        let left = {
            let state = ButtonState::from((byte & L3_MASK) >> L3_MASK.trailing_zeros());
            let coordinates = StickCoord {
                x: self.packet.borrow()[1],
                y: self.packet.borrow()[2],
            };
            StickState { state, coordinates }
        };
        let right = {
            let state = ButtonState::from((byte & R3_MASK) >> R3_MASK.trailing_zeros());
            let coordinates = StickCoord {
                x: self.packet.borrow()[3],
                y: self.packet.borrow()[4],
            };
            StickState { state, coordinates }
        };

        dbg!(&left);
        dbg!(&right);

        AnalogState { left, right }
    }
}

impl Drop for DualSense {
    fn drop(&mut self) {
        // TODO: Unwrapping is not the best choice, maybe do something else?
        crate::hidapi::exit().unwrap();
    }
}
