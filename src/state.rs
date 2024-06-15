//! The DualSense state module.
//!
//! This module contains the implementations of (almost) all possible controller states.

/// The state of the action buttons.
#[derive(Debug)]
pub struct ActionState {
    /// The state of the cross button.
    pub cross: ButtonState,
    /// The state of the square button.
    pub square: ButtonState,
    /// The state of the triangle button.
    pub triangle: ButtonState,
    /// The state of the circle button.
    pub circle: ButtonState,
}

/// The state of the directional buttons.
#[derive(Debug)]
pub struct DirectionalState {
    /// The state of the bottom directional button.
    pub bottom: ButtonState,
    /// The state of the left directional button.
    pub left: ButtonState,
    /// The state of the top directional button.
    pub top: ButtonState,
    /// The state of the right directional button.
    pub right: ButtonState,
}

/// The state of the triggers.
#[derive(Debug)]
pub struct TriggerState {
    /// The state of the L1 trigger.
    pub l1: Trigger,
    /// The state of the R1 trigger.
    pub r1: Trigger,
    /// The state of the L2 trigger.
    pub l2: Trigger,
    /// The state of the R2 trigger.
    pub r2: Trigger,
}

/// The state of a trigger.
#[derive(Debug)]
pub struct Trigger {
    /// Whether the trigger is pressed or not.
    pub state: ButtonState,
    /// The axis (i.e. pressed amount) of the trigger.
    ///
    /// Note that the only triggers that have axis are L2 and R2.
    pub axis: Option<Axis>,
}

/// The axis of a trigger.
#[derive(Debug)]
pub struct Axis(pub u8);

/// The state of a button.
#[derive(Debug)]
pub enum ButtonState {
    /// Button is pressed.
    Released,
    /// Button is released.
    Pressed,
}

/// The state of the analog sticks.
#[derive(Debug)]
pub struct AnalogState {
    /// The state of the left analog.
    pub left: StickState,
    /// The state of the right analog.
    pub right: StickState,
}

/// The state of a analog stick.
#[derive(Debug)]
pub struct StickState {
    /// Whether the stick is pressed or not.
    pub state: ButtonState,
    /// The coordinates of the stick.
    pub coordinates: StickCoord,
}

/// The Coordinates of a stick.
#[derive(Debug)]
pub struct StickCoord {
    /// The x coordinate.
    pub x: u8,
    /// The y coordinate.
    pub y: u8,
}

impl From<u8> for ButtonState {
    fn from(value: u8) -> Self {
        match value {
            0 => ButtonState::Released,
            1 => ButtonState::Pressed,
            _ => unreachable!(),
        }
    }
}
