//! The HIDAPI FFI module.
//!
//! This module interacts with the HIDAPI library. This library enables the interaction with HID
//! (Human Interface Device) peripherals through well-defined functions.
//!
//! # About the implementation
//! The HIDAPI library has multiple functions, each used for different purposes. Even so, the only
//! binded functions are the ones necessary for communicating with the DualSense controller. This
//! module *does not* aim to be a replacement for a full implementation like other known crates.
//! The goal of not using such crates is to minimize dependencies while still being able to
//! interact with the HIDAPI defined functions.

// TODO: Improve platform support (MacOS, Windows, Linux, FreeBSD).
// TODO: Improve error types (mirror HID error messages).

mod ffi;

use crate::Mode;

use libc::c_int;
use std::ptr;
use thiserror::Error;

/// A HID device vendor ID.
///
/// A vendor ID is an identifier that distinguishes manufacturers from each other. The DualSense
/// controller vendor ID is 0x054C (16-bits).
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct VendorID(u16);

impl VendorID {
    pub(crate) const fn new(id: u16) -> Self {
        VendorID(id)
    }

    pub(crate) const fn id(&self) -> u16 {
        self.0
    }
}

/// A HID device product ID.
///
/// A product ID is an identifier that distinguishes products made from the same manufacturer from
/// each other. The DualSense controller product ID is 0x0CE6 (16-bits).
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ProductID(u16);

impl ProductID {
    pub(crate) const fn new(id: u16) -> Self {
        ProductID(id)
    }

    pub(crate) const fn id(&self) -> u16 {
        self.0
    }
}

/// A wrapper for a HIDAPI HID Device.
pub(crate) struct DeviceWrapper {
    device: *mut ffi::hid_device,
}

impl DeviceWrapper {
    /// Try to open a HID device.
    ///
    /// This function can fail. The most common reason is if a device with the supplied `vendor_id`
    /// and `product_id` was not found.
    pub(crate) fn open(vendor_id: VendorID, product_id: ProductID) -> Result<Self, Error> {
        // SAFETY: This is safe since we only supply `unsigned short` variables to the function.
        // The function `hid_open` returns a `null` pointer in the fail case. This is handled in
        // the `if` below.
        let device = unsafe { ffi::hid_open(vendor_id.id(), product_id.id(), ptr::null()) };
        if device.is_null() {
            return Err(Error::Open);
        }

        Ok(DeviceWrapper { device })
    }

    /// Set the HID device mode to be either _blocking_ or _non-blocking_.
    ///
    /// See the [`Mode`] enum for more information.
    ///
    /// [`Mode`]: enum@crate::Mode
    pub(crate) fn set_mode(&self, mode: Mode) -> Result<(), Error> {
        // SAFETY: This function is safe to call since the device is guaranteed to be not `null`,
        // as the only way to get one is by calling `open`, and we check if the pointer is valid
        // during it. Also, the conversion of `Mode` to `c_int` is safe since the enum has
        // well-defined values (0 for blocking and 1 for non-blocking). This function returns `-1`
        // in case of an error and `0` otherwise.
        match unsafe { ffi::hid_set_nonblocking(self.device, mode as c_int) } {
            -1 => Err(Error::Mode),
            0 => Ok(()),
            _ => unreachable!(),
        }
    }

    /// Read data from a HID device to `buf`.
    ///
    /// This function returns the number of bytes read in case of success.
    pub(crate) fn read(&self, buf: &mut [u8]) -> Result<usize, Error> {
        // SAFETY: This function is safe to call since the device is guaranteed to be not `null`,
        // as the only way to get one is by calling `open`, and we check if the pointer is valid
        // during it. Also, the slice `buf` outlives the created mutable pointer. It is also
        // guaranteed to not have buffer overflows since we pass the correct buffer length to it.
        match unsafe { ffi::hid_read(self.device, buf.as_mut_ptr(), buf.len()) } {
            -1 => Err(Error::Read),
            bytes => Ok(bytes as usize),
        }
    }
}

impl Drop for DeviceWrapper {
    fn drop(&mut self) {
        // SAFETY: This is safe, since we know `self.device` is a valid device, as the only way to
        // get one is by calling `open`, and we check if the pointer is valid during it.
        unsafe {
            ffi::hid_close(self.device);
        }
    }
}

/// Finalize the HIDAPI library.
///
/// This function frees all of the static data associated with `HIDAPI`. It should be called when
/// the `HIDAPI` library is not needed anymore to avoid memory leaks.
pub(crate) fn exit() -> Result<(), Error> {
    // SAFETY: This function is safe to call since we handle all the possible cases (`-1` for error
    // and `0` for success).
    match unsafe { ffi::hid_exit() } {
        -1 => Err(Error::Exit),
        0 => Ok(()),
        _ => unreachable!(),
    }
}

/// A raw representation of an input report from a DualSense controller using a USB connection.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct RawInputReportUSB([u8; 64]);

impl RawInputReportUSB {
    pub(crate) fn new(slice: [u8; 64]) -> Self {
        RawInputReportUSB(slice)
    }

    pub(crate) fn as_array(&self) -> &[u8; 64] {
        &self.0
    }
}

/// The error type for operations with a HID device.
#[derive(Error, Debug)]
pub(crate) enum Error {
    /// An open error.
    ///
    /// This error can happen when trying to open a HID device.
    #[error("Could not open HID device")]
    Open,
    /// A mode error.
    ///
    /// This error can happen when trying to change the update mode (_blocking_ or _non-blocking_)
    /// of the controller.
    #[error("Could not change mode")]
    Mode,
    /// A read error.
    ///
    /// This error can happen when trying to read from a HID device.
    #[error("Could not read HID device")]
    Read,
    /// An exit error.
    ///
    /// This error can happen when trying to finish using the controller (usually when dropping
    /// it).
    #[error("Could not properly clean up at controller exit")]
    Exit,
}
