// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceAdsl")]
    pub struct DeviceAdsl(Object<ffi::NMDeviceAdsl, ffi::NMDeviceAdslClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_adsl_get_type(),
    }
}

impl DeviceAdsl {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has carrier
    #[doc(alias = "nm_device_adsl_get_carrier")]
    #[doc(alias = "get_carrier")]
    pub fn is_carrier(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_adsl_get_carrier(self.to_glib_none().0)) }
    }

    #[doc(alias = "carrier")]
    pub fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceAdsl) + 'static>(
            this: *mut ffi::NMDeviceAdsl,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceAdsl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceAdsl")
    }
}
