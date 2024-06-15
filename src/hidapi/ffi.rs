use libc::{c_int, c_uchar, c_ushort, c_void, size_t, wchar_t};
use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
#[allow(non_camel_case_types)]
pub(super) struct hid_device {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

extern "C" {
    /// @brief Open a HID device using a Vendor ID (VID), Product ID (PID) and optionally a
    /// serial number.
    ///
    /// If @p serial_number is NULL, the first device with the specified VID and PID is opened.
    ///
    /// @param vendor_id The Vendor ID (VID) of the device to open.
    /// @param product_id The Product ID (PID) of the device to open.
    /// @param serial_number The Serial Number of the device to open (Optionally NULL).
    ///
    /// @returns This function returns a pointer to a #hid_device object on success or NULL on
    /// failure. Call hid_error(NULL) to get the failure reason.
    ///
    /// @note The returned object must be freed by calling hid_close(), when not needed anymore.
    pub(super) fn hid_open(
        vendor_id: c_ushort,
        product_id: c_ushort,
        serial_number: *const wchar_t,
    ) -> *mut hid_device;

    /// @brief Close a HID device.
    ///
    /// @param dev A device handle returned from hid_open().
    pub(super) fn hid_close(dev: *mut hid_device) -> c_void;

    /// @brief Read an Input report from a HID device.
    ///
    /// Input reports are returned to the host through the INTERRUPT IN endpoint. The first byte
    /// will contain the Report number if the device uses numbered reports.
    ///
    /// @param dev A device handle returned from hid_open().
    /// @param data A buffer to put the read data into.
    /// @param length The number of bytes to read. For devices with multiple reports, make sure to
    /// read an extra byte for the report number.
    ///
    /// @returns This function returns the actual number of bytes read and -1 on error.
    /// Call hid_error(dev) to get the failure reason. If no packet was available to be read and
    /// the handle is in non-blocking mode, this function returns 0.
    pub(super) fn hid_read(dev: *mut hid_device, data: *mut c_uchar, length: size_t) -> c_int;

    /// @brief Set the device handle to be non-blocking.
    ///
    /// In non-blocking mode calls to hid_read() will return immediately with a value of 0 if there
    /// is no data to be read. In blocking mode, hid_read() will wait (block) until there is data
    /// to read before returning.
    ///
    /// Nonblocking can be turned on and off at any time.
    ///
    /// @param dev A device handle returned from hid_open().
    /// @param nonblock enable or not the nonblocking reads
    ///  - 1 to enable nonblocking
    ///  - 0 to disable nonblocking.
    ///
    /// @returns This function returns 0 on success and -1 on error.
    /// Call hid_error(dev) to get the failure reason.
    pub(super) fn hid_set_nonblocking(dev: *mut hid_device, nonblock: c_int) -> c_int;

    /// @brief Finalize the HIDAPI library.
    ///
    /// This function frees all of the static data associated with HIDAPI. It should be called at
    /// the end of execution to avoid memory leaks.
    ///
    /// @returns This function returns 0 on success and -1 on error.
    pub(super) fn hid_exit() -> c_int;
}
