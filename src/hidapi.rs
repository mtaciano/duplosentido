//! The HIDAPI FFI module.
//!
//! This module interacts with HIDAPI (a C library) to facilitate the communication with the
//! DualSense controller.
//!
//! Note that the only implemented functions are the ones necessary for communicating with the
//! controller. This module *does not* aim to be a replacement for a full implementation like other
//! crates. The only objetive is to minimize dependencies while also keeping interactions with the
//! library possible.

mod ffi;

use libc::c_int;
use std::ptr;
use thiserror::Error;

pub(crate) struct VendorID(pub(crate) u16);
pub(crate) struct ProductID(pub(crate) u16);

pub(crate) struct HidDevice {
    device: *mut ffi::hid_device,
}

pub(crate) enum Mode {
    Blocking = 0,
    // NonBlocking = 1,
}

impl HidDevice {
    /// Open a HID device.
    pub(crate) fn open(vendor_id: &VendorID, product_id: &ProductID) -> Result<Self, HidError> {
        // SAFETY:
        // This function returns `NULL` if it failed to open a hid device. Else, the device was
        // created successfully.
        let device: *mut ffi::hid_device =
            unsafe { ffi::hid_open(vendor_id.0, product_id.0, ptr::null()) };

        match device.is_null() {
            true => Err(HidError::Open),
            false => Ok(HidDevice { device }),
        }
    }

    /// Set the device handle to be either blocking or non-blocking.
    pub(crate) fn set_mode(&self, nonblock: Mode) -> Result<(), HidError> {
        // SAFETY:
        // This function is safe to use.
        match unsafe { ffi::hid_set_nonblocking(self.device, nonblock as c_int) } {
            -1 => Err(HidError::Block),
            0 => Ok(()),
            _ => unreachable!(),
        }
    }

    /// Read an input report from a HID device.
    pub(crate) fn read(&self, buf: &mut [u8]) -> Result<usize, HidError> {
        // SAFETY:
        // It is safe to call this function.
        match unsafe { ffi::hid_read(self.device, buf.as_mut_ptr(), buf.len()) } {
            -1 => Err(HidError::Read),
            read => Ok(read as usize),
        }
    }
}

impl Drop for HidDevice {
    fn drop(&mut self) {
        // SAFETY:
        // It is safe to call `hid_close` since we know we own a `hid_device` and it is guaranteed
        // to not be null.
        unsafe {
            ffi::hid_close(self.device);
        }
    }
}

/// Finalize the HIDAPI library.
pub(crate) fn exit() -> Result<(), HidError> {
    match unsafe { ffi::hid_exit() } {
        -1 => Err(HidError::Finalize),
        0 => Ok(()),
        _ => unreachable!(),
    }
}

/// The possible error for HID devices.
#[derive(Error, Debug)]
pub enum HidError {
    /// Open error.
    #[error("Could not open device")]
    Open,
    /// Block error.
    #[error("Could not change block mode")]
    Block,
    /// Read error.
    #[error("Could not read hid device")]
    Read,
    /// Finalize error.
    #[error("Could not finalize the library")]
    Finalize,
}
