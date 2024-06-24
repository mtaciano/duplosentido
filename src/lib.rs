//! A crate for easy interactions with the DualSense (PS5) controller.
//!
//! This crate creates an easy way of interacting with the DualSense controller. The main
//! motivation for its existence is the fact that, without it, interacting with the controller is a
//! _very_ cerimonial and error prone endeavour. Our objetive is handling the necessary details
//! while exposing simple and intuitive ways of interacting with the controller.
//!
//! # How to use
//! To use this crate, you should bind to a [`DualSense`] controller:
//!
//! ```rust
//! use duplosentido::DualSense;
//!
//! let ds = DualSense::bind().expect("At least one controller should be connected");
//! ```
//!
//! After you've successfully binded with a controller, you need to call [`update`] to get the
//! latest input given by it. Note that, by default, the controller is started in _blocking_ mode.
//! If _non-blocking_ mode is better suited for your needs, you can call [`set_mode`] to change
//! the behavior.
//!
//! With the controller state updated, you can call [`state`] to get its latest state. With it in
//! hands, you now have the latest snapshot of the inputs made by the controller. With it, you can
//! query its buttons easily. A full _blocking_ example can be viewed below:
//!
//! ```rust
//! use duplosentido::DualSense;
//!
//! let ds = DualSense::bind().expect("At least one controller should be connected");
//!
//! // `update()` returns the number of bytes read (It can be 0 in non-blocking mode).
//! let _ = ds.update().unwrap();
//! let controller = ds.state();
//!
//! if controller.square().is_pressed() {
//!     println!("Square is being pressed!");
//! }
//! ```
//!
//! # Roadmap
//! This crate is still a major work in progress. Below you can see the "roadmap" for this crate,
//! in no particular order:
//! - [x] Linux support.
//! - [ ] Windows and macOS support.
//! - [x] USB connection support.
//! - [ ] Bluetooth connection support.
//! - [ ] Adaptive trigger support.
//! - [ ] Vibration support.
//! - [ ] No dependencies (maybe?).
//!
//! [`DualSense`]: struct@crate::DualSense
//! [`update`]: fn@crate::DualSense::update
//! [`set_mode`]: fn@crate::DualSense::set_mode
//! [`state`]: fn@crate::DualSense::state

#![warn(missing_docs)]

pub(crate) mod hidapi;

pub mod mappings;

mod dualsense;
pub use crate::dualsense::{DualSense, DualSenseState, Error, Mode, Result};
