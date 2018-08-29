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

glib_wrapper! {
    pub struct DeviceTun(Object<ffi::NMDeviceTun, ffi::NMDeviceTunClass>): Device;

    match fn {
        get_type => || ffi::nm_device_tun_get_type(),
    }
}

pub trait DeviceTunExt {
    fn get_group(&self) -> i64;

    fn get_mode(&self) -> Option<String>;

    fn get_multi_queue(&self) -> bool;

    fn get_no_pi(&self) -> bool;

    fn get_owner(&self) -> i64;

    fn get_vnet_hdr(&self) -> bool;

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceTun> + IsA<glib::object::Object>> DeviceTunExt for O {
    fn get_group(&self) -> i64 {
        unsafe { ffi::nm_device_tun_get_group(self.to_glib_none().0) }
    }

    fn get_mode(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_device_tun_get_mode(self.to_glib_none().0)) }
    }

    fn get_multi_queue(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_multi_queue(self.to_glib_none().0)) }
    }

    fn get_no_pi(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_no_pi(self.to_glib_none().0)) }
    }

    fn get_owner(&self) -> i64 {
        unsafe { ffi::nm_device_tun_get_owner(self.to_glib_none().0) }
    }

    fn get_vnet_hdr(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_tun_get_vnet_hdr(self.to_glib_none().0)) }
    }

    fn connect_property_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::group",
                transmute(notify_group_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::multi-queue",
                transmute(notify_multi_queue_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_no_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::no-pi",
                transmute(notify_no_pi_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::owner",
                transmute(notify_owner_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::vnet-hdr",
                transmute(notify_vnet_hdr_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_group_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hw_address_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mode_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_multi_queue_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_no_pi_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_owner_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vnet_hdr_trampoline<P>(
    this: *mut ffi::NMDeviceTun,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceTun>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTun::from_glib_borrow(this).downcast_unchecked())
}
