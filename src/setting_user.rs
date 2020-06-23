// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use glib;
use glib::object::Cast;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_8", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    pub struct SettingUser(Object<nm_sys::NMSettingUser, nm_sys::NMSettingUserClass, SettingUserClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_user_get_type(),
    }
}

impl SettingUser {
    /// Creates a new `SettingUser` object with default values.
    ///
    /// # Returns
    ///
    /// the new empty `SettingUser` object
    pub fn new() -> SettingUser {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_user_new()).unsafe_cast() }
    }

    ///
    /// Feature: `v1_8`
    ///
    /// ## `key`
    /// the key to lookup
    ///
    /// # Returns
    ///
    /// the value associated with `key` or `None` if no such
    ///  value exists.
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn get_data(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_user_get_data(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    /// ## `out_len`
    /// the length of the returned array
    ///
    /// # Returns
    ///
    /// a
    ///  `None`-terminated array containing each key from the table.
    pub fn get_keys(&self) -> Vec<GString> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                nm_sys::nm_setting_user_get_keys(self.to_glib_none().0, out_len.as_mut_ptr()),
                out_len.assume_init() as usize,
            );
            ret
        }
    }

    ///
    /// Feature: `v1_8`
    ///
    /// ## `key`
    /// the key to set
    /// ## `val`
    /// the value to set or `None` to clear a key.
    ///
    /// # Returns
    ///
    /// `true` if the operation was successful. The operation
    ///  can fail if `key` or `val` are not valid strings according
    ///  to `SettingUser::check_key` and `SettingUser::check_val`.
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn set_data(&self, key: &str, val: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_user_set_data(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Checks whether `key` is a valid user data key. This means,
    /// key is not `None`, not too large and valid ASCII. Also,
    /// only digits and numbers are allowed with a few special
    /// characters. The key must contain at least one '.' and
    /// look like a fully qualified DNS name.
    ///
    /// Feature: `v1_8`
    ///
    /// ## `key`
    /// the key to check
    ///
    /// # Returns
    ///
    /// `true` if `key` is a valid user data key.
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn check_key(key: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_user_check_key(key.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Checks whether `val` is a valid user data value. This means,
    /// value is not `None`, not too large and valid UTF-8.
    ///
    /// Feature: `v1_8`
    ///
    /// ## `val`
    /// the value to check
    ///
    /// # Returns
    ///
    /// `true` if `val` is a valid user data value.
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn check_val(val: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_setting_user_check_val(val.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn connect_property_data_notify<F: Fn(&SettingUser) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_data_trampoline<F: Fn(&SettingUser) + 'static>(
            this: *mut nm_sys::NMSettingUser,
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
                b"notify::data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingUser {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingUser")
    }
}
