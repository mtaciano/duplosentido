//! The mappings for the DualSense controller.
//!
//! This layout provides an intuitive, ergonomic and type-safe experience for all the inputs of the
//! DualSense controller. Buttons and parts are represented as states, which are snapshots of a
//! controller components as it was during its last update.

pub(crate) mod group;

mod state;
pub use state::{
    AccelerationState, AngularVelocityState, BackTriggerEffect, BackTriggerState,
    BackTriggerStatus, ButtonState, DPadDirection, MicrophoneState, MutedState, PluggedState,
    PowerState, StickState, TemperatureState, TouchPadState, USBState,
};
pub(crate) use state::{Axis, BackTriggerStop, FingerData, StickCoordinates};
