//! The DualSense controller crate.
//!
//! The objective of this crate is making it possible to interact with the dualsense controller
//! easily. This crate currently is a work in progress, so you should not use in in production
//! environments, for more information see [`DualSense`].
#![warn(missing_docs)]

pub mod dualsense;
pub mod hidapi;
pub mod state;

pub use dualsense::*;
