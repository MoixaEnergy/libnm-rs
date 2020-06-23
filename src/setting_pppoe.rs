// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
use crate::SettingSecretFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SettingPppoe(Object<nm_sys::NMSettingPppoe, nm_sys::NMSettingPppoeClass, SettingPppoeClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_pppoe_get_type(),
    }
}

impl SettingPppoe {
    /// Creates a new `SettingPppoe` object with default values.
    ///
    /// # Returns
    ///
    /// the new empty `SettingPppoe` object
    pub fn new() -> SettingPppoe {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_pppoe_new()).unsafe_cast() }
    }
}

impl Default for SettingPppoe {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_PPPOE: Option<&SettingPppoe> = None;

/// Trait containing all `SettingPppoe` methods.
///
/// # Implementors
///
/// [`SettingPppoe`](struct.SettingPppoe.html)
pub trait SettingPppoeExt: 'static {
    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingPppoe:parent` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_parent(&self) -> Option<GString>;

    ///
    /// # Returns
    ///
    /// the `SettingPppoe:password` property of the setting
    fn get_password(&self) -> Option<GString>;

    ///
    /// # Returns
    ///
    /// the `SettingSecretFlags` pertaining to the `SettingPppoe:password`
    fn get_password_flags(&self) -> SettingSecretFlags;

    ///
    /// # Returns
    ///
    /// the `SettingPppoe:service` property of the setting
    fn get_service(&self) -> Option<GString>;

    ///
    /// # Returns
    ///
    /// the `SettingPppoe:username` property of the setting
    fn get_username(&self) -> Option<GString>;

    /// If given, specifies the parent interface name on which this PPPoE
    /// connection should be created. If this property is not specified,
    /// the connection is activated on the interface specified in
    /// `SettingConnection:interface-name` of `SettingConnection`.
    ///
    /// Feature: `v1_10`
    ///
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>);

    /// Password used to authenticate with the PPPoE service.
    fn set_property_password(&self, password: Option<&str>);

    /// Flags indicating how to handle the `SettingPppoe:password` property.
    fn set_property_password_flags(&self, password_flags: SettingSecretFlags);

    /// If specified, instruct PPPoE to only initiate sessions with access
    /// concentrators that provide the specified service. For most providers,
    /// this should be left blank. It is only required if there are multiple
    /// access concentrators or a specific service is known to be required.
    fn set_property_service(&self, service: Option<&str>);

    /// Username used to authenticate with the PPPoE service.
    fn set_property_username(&self, username: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingPppoe>> SettingPppoeExt for O {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_parent(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_pppoe_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_password(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_pppoe_get_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_password_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(nm_sys::nm_setting_pppoe_get_password_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_service(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_pppoe_get_service(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_username(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_pppoe_get_username(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"parent\0".as_ptr() as *const _,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    fn set_property_password(&self, password: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"password\0".as_ptr() as *const _,
                Value::from(password).to_glib_none().0,
            );
        }
    }

    fn set_property_password_flags(&self, password_flags: SettingSecretFlags) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"password-flags\0".as_ptr() as *const _,
                Value::from(&password_flags).to_glib_none().0,
            );
        }
    }

    fn set_property_service(&self, service: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"service\0".as_ptr() as *const _,
                Value::from(service).to_glib_none().0,
            );
        }
    }

    fn set_property_username(&self, username: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"username\0".as_ptr() as *const _,
                Value::from(username).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingPppoe,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingPppoe>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingPppoe::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingPppoe,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingPppoe>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingPppoe::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingPppoe,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingPppoe>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingPppoe::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingPppoe,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingPppoe>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingPppoe::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::service\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_service_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingPppoe,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingPppoe>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingPppoe::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::username\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_username_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingPppoe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingPppoe")
    }
}
