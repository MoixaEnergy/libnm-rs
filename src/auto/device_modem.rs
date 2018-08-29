// This file was generated by gir (https://github.com/gtk-rs/gir @ 609779c)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Device;
use DeviceModemCapabilities;

glib_wrapper! {
    pub struct DeviceModem(Object<ffi::NMDeviceModem, ffi::NMDeviceModemClass>): Device;

    match fn {
        get_type => || ffi::nm_device_modem_get_type(),
    }
}

pub trait DeviceModemExt {
    fn get_current_capabilities(&self) -> DeviceModemCapabilities;

    fn get_modem_capabilities(&self) -> DeviceModemCapabilities;

    fn connect_property_current_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_modem_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DeviceModem> + IsA<glib::object::Object>> DeviceModemExt for O {
    fn get_current_capabilities(&self) -> DeviceModemCapabilities {
        unsafe {
            from_glib(ffi::nm_device_modem_get_current_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_modem_capabilities(&self) -> DeviceModemCapabilities {
        unsafe {
            from_glib(ffi::nm_device_modem_get_modem_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    fn connect_property_current_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::current-capabilities",
                transmute(notify_current_capabilities_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_modem_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::modem-capabilities",
                transmute(notify_modem_capabilities_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_current_capabilities_trampoline<P>(
    this: *mut ffi::NMDeviceModem,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceModem>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceModem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_modem_capabilities_trampoline<P>(
    this: *mut ffi::NMDeviceModem,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceModem>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceModem::from_glib_borrow(this).downcast_unchecked())
}
