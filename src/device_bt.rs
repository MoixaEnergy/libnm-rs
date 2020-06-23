// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BluetoothCapabilities;
use crate::Device;
use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceBt(Object<nm_sys::NMDeviceBt, nm_sys::NMDeviceBtClass, DeviceBtClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_bt_get_type(),
    }
}

impl DeviceBt {
    /// Returns the Bluetooth device's usable capabilities.
    ///
    /// # Returns
    ///
    /// a combination of `BluetoothCapabilities`
    pub fn get_capabilities(&self) -> BluetoothCapabilities {
        unsafe { from_glib(nm_sys::nm_device_bt_get_capabilities(self.to_glib_none().0)) }
    }

    /// Gets the name of the `DeviceBt`.
    ///
    /// # Returns
    ///
    /// the name of the device
    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_bt_get_name(self.to_glib_none().0)) }
    }

    /// The device's bluetooth capabilities, a combination of `BluetoothCapabilities`.
    pub fn get_property_bt_capabilities(&self) -> BluetoothCapabilities {
        unsafe {
            let mut value = Value::from_type(<BluetoothCapabilities as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"bt-capabilities\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `bt-capabilities` getter")
                .unwrap()
        }
    }

    pub fn connect_property_bt_capabilities_notify<F: Fn(&DeviceBt) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bt_capabilities_trampoline<F: Fn(&DeviceBt) + 'static>(
            this: *mut nm_sys::NMDeviceBt,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bt-capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bt_capabilities_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_name_notify<F: Fn(&DeviceBt) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&DeviceBt) + 'static>(
            this: *mut nm_sys::NMDeviceBt,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceBt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceBt")
    }
}
