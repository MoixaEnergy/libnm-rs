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
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;
use SettingMacvlanMode;

glib_wrapper! {
    pub struct SettingMacvlan(Object<ffi::NMSettingMacvlan, ffi::NMSettingMacvlanClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_macvlan_get_type(),
    }
}

impl SettingMacvlan {
    pub fn new() -> SettingMacvlan {
        unsafe { Setting::from_glib_full(ffi::nm_setting_macvlan_new()).downcast_unchecked() }
    }
}

impl Default for SettingMacvlan {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingMacvlanExt {
    fn get_mode(&self) -> SettingMacvlanMode;

    fn get_parent(&self) -> Option<String>;

    fn get_promiscuous(&self) -> bool;

    fn get_tap(&self) -> bool;

    fn set_property_mode(&self, mode: u32);

    fn set_property_parent(&self, parent: Option<&str>);

    fn set_property_promiscuous(&self, promiscuous: bool);

    fn set_property_tap(&self, tap: bool);

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_promiscuous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingMacvlan> + IsA<glib::object::Object>> SettingMacvlanExt for O {
    fn get_mode(&self) -> SettingMacvlanMode {
        unsafe { from_glib(ffi::nm_setting_macvlan_get_mode(self.to_glib_none().0)) }
    }

    fn get_parent(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_macvlan_get_parent(self.to_glib_none().0)) }
    }

    fn get_promiscuous(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_macvlan_get_promiscuous(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_tap(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_macvlan_get_tap(self.to_glib_none().0)) }
    }

    fn set_property_mode(&self, mode: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mode".to_glib_none().0,
                Value::from(&mode).to_glib_none().0,
            );
        }
    }

    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "parent".to_glib_none().0,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    fn set_property_promiscuous(&self, promiscuous: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "promiscuous".to_glib_none().0,
                Value::from(&promiscuous).to_glib_none().0,
            );
        }
    }

    fn set_property_tap(&self, tap: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "tap".to_glib_none().0,
                Value::from(&tap).to_glib_none().0,
            );
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

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::parent",
                transmute(notify_parent_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_promiscuous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::promiscuous",
                transmute(notify_promiscuous_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_tap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::tap",
                transmute(notify_tap_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_mode_trampoline<P>(
    this: *mut ffi::NMSettingMacvlan,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingMacvlan>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingMacvlan::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_trampoline<P>(
    this: *mut ffi::NMSettingMacvlan,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingMacvlan>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingMacvlan::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_promiscuous_trampoline<P>(
    this: *mut ffi::NMSettingMacvlan,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingMacvlan>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingMacvlan::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tap_trampoline<P>(
    this: *mut ffi::NMSettingMacvlan,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingMacvlan>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingMacvlan::from_glib_borrow(this).downcast_unchecked())
}
