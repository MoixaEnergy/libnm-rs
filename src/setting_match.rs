// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct SettingMatch(Object<nm_sys::NMSettingMatch, nm_sys::NMSettingMatchClass, SettingMatchClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_match_get_type(),
    }
}

impl SettingMatch {
    /// Creates a new `SettingMatch` object with default values.
    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingMatch` object
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> SettingMatch {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_match_new()).unsafe_cast() }
    }

    /// Adds a new interface name to the setting.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `interface_name`
    /// the interface name to add
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn add_interface_name(&self, interface_name: &str) {
        unsafe {
            nm_sys::nm_setting_match_add_interface_name(
                self.to_glib_none().0,
                interface_name.to_glib_none().0,
            );
        }
    }

    /// Removes all configured interface names.
    ///
    /// Feature: `v1_14`
    ///
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn clear_interface_names(&self) {
        unsafe {
            nm_sys::nm_setting_match_clear_interface_names(self.to_glib_none().0);
        }
    }

    ///
    /// Feature: `v1_14`
    ///
    /// ## `idx`
    /// index number of the DNS search domain to return
    ///
    /// # Returns
    ///
    /// the interface name at index `idx`
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_interface_name(&self, idx: i32) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_match_get_interface_name(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    /// Returns all the interface names.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `length`
    /// the length of the returned interface names array.
    ///
    /// # Returns
    ///
    /// the configured interface names.
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_interface_names(&self) -> Vec<GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                nm_sys::nm_setting_match_get_interface_names(
                    self.to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            );
            ret
        }
    }

    ///
    /// Feature: `v1_14`
    ///
    ///
    /// # Returns
    ///
    /// the number of configured interface names
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_num_interface_names(&self) -> u32 {
        unsafe { nm_sys::nm_setting_match_get_num_interface_names(self.to_glib_none().0) }
    }

    /// Removes the interface name at index `idx`.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `idx`
    /// index number of the interface name
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_interface_name(&self, idx: i32) {
        unsafe {
            nm_sys::nm_setting_match_remove_interface_name(self.to_glib_none().0, idx);
        }
    }

    /// Removes `interface_name`.
    ///
    /// Feature: `v1_14`
    ///
    /// ## `interface_name`
    /// the interface name to remove
    ///
    /// # Returns
    ///
    /// `true` if the interface name was found and removed; `false` if it was not.
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn remove_interface_name_by_value(&self, interface_name: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_match_remove_interface_name_by_value(
                self.to_glib_none().0,
                interface_name.to_glib_none().0,
            ))
        }
    }

    /// A list of interface names to match. Each element is a shell wildcard
    /// pattern. When an element is prefixed with exclamation mark (!) the
    /// condition is inverted.
    ///
    /// A candidate interface name is considered matching when both these
    /// conditions are satisfied: (a) any of the elements not prefixed with '!'
    /// matches or there aren't such elements; (b) none of the elements
    /// prefixed with '!' match.
    ///
    /// Feature: `v1_14`
    ///
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_property_interface_name(&self, interface_name: &[&str]) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"interface-name\0".as_ptr() as *const _,
                Value::from(interface_name).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn connect_property_interface_name_notify<F: Fn(&SettingMatch) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_interface_name_trampoline<F: Fn(&SettingMatch) + 'static>(
            this: *mut nm_sys::NMSettingMatch,
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
                b"notify::interface-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interface_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl Default for SettingMatch {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingMatch")
    }
}
