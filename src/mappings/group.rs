//! The groups of states.
//!
//! Groups are the collection of closely related states.

use crate::mappings::{
    BackTriggerState, ButtonState, MicrophoneState, PluggedState, PowerState, StickState, USBState,
};

/// A group of the action buttons.
///
/// Action buttons are defined as the buttons present in the right part of the controller,
/// represented as symbols (square, triangle, circle and cross to be exact).
#[derive(Debug, Copy, Clone)]
pub(crate) struct ActionButtonGroup {
    /// A state of the square button.
    pub(crate) square: ButtonState,
    /// A state of the triangle button.
    pub(crate) triangle: ButtonState,
    /// A state of the circle button.
    pub(crate) circle: ButtonState,
    /// A state of the cross button.
    pub(crate) cross: ButtonState,
}

/// A group of the front triggers.
///
/// Front triggers are defined as the front buttons on top of the controller, labeled L1 and R1.
#[derive(Debug, Copy, Clone)]
pub(crate) struct FrontTriggerGroup {
    /// A state of the L1 trigger button.
    pub(crate) l1: ButtonState,
    /// A state of the R1 trigger button.
    pub(crate) r1: ButtonState,
}

/// A group of the back triggers.
///
/// Back triggers are defined as the back buttons on top of the controller, labeled L2 and R2.
#[derive(Debug, Copy, Clone)]
pub(crate) struct BackTriggerGroup {
    /// A state of the L2 trigger button.
    pub(crate) l2: BackTriggerState,
    /// A state of the R2 trigger button.
    pub(crate) r2: BackTriggerState,
}

/// A analog stick group.
#[derive(Debug, Copy, Clone)]
pub(crate) struct StickGroup {
    /// Left analog stick (L3).
    pub(crate) left: StickState,
    /// Right analog stick (R3).
    pub(crate) right: StickState,
}

/// A menu button group.
#[derive(Debug, Copy, Clone)]
pub(crate) struct MenuGroup {
    /// Create button.
    pub(crate) create: ButtonState,
    /// Options button.
    pub(crate) options: ButtonState,
    /// Home (PS) button.
    pub(crate) home: ButtonState,
    /// Mute button.
    pub(crate) mute: ButtonState,
}

/// A group of the controller power.
#[derive(Debug, Copy, Clone)]
pub(crate) struct PowerGroup {
    /// The state of the power.
    pub(crate) state: PowerState,
    /// Controller charged percent.
    pub(crate) percent: u8,
}

/// A group of external plugged devices.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct PluggedGroup {
    /// The state of the headphone.
    pub(crate) headphone: PluggedState,
    /// The state of the microphone.
    pub(crate) microphone: MicrophoneState,
    /// The state of the USB.
    pub(crate) usb: USBState,
    // Allow unused while I try to understand it.
    // TODO: Understand it.
    #[allow(unused)]
    pub(crate) haptic_low_pass_filter: PluggedState,
}
