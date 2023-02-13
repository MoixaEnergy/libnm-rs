// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::fmt;
#[cfg(any(feature = "v1_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
use std::{boxed::Box as Box_, mem, mem::transmute, ptr};

glib::wrapper! {
    /// OVS External IDs Settings
    ///
    /// ## Properties
    ///
    ///
    /// #### `data`
    ///  A dictionary of key/value pairs with exernal-ids for OVS.
    ///
    /// Readable | Writeable
    /// <details><summary><h4>Setting</h4></summary>
    ///
    ///
    /// #### `name`
    ///  The setting's name, which uniquely identifies the setting within the
    /// connection. Each setting type has a name unique to that type, for
    /// example "ppp" or "802-11-wireless" or "802-3-ethernet".
    ///
    /// Readable
    /// </details>
    ///
    /// # Implements
    ///
    /// [`SettingExt`][trait@crate::prelude::SettingExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMSettingOvsExternalIDs")]
    pub struct SettingOvsExternalIDs(Object<ffi::NMSettingOvsExternalIDs, ffi::NMSettingOvsExternalIDsClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_ovs_external_ids_get_type(),
    }
}

impl SettingOvsExternalIDs {
    /// Creates a new [`SettingOvsExternalIDs`][crate::SettingOvsExternalIDs] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty
    /// [`SettingOvsExternalIDs`][crate::SettingOvsExternalIDs] object
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_new")]
    pub fn new() -> SettingOvsExternalIDs {
        unsafe { from_glib_full(ffi::nm_setting_ovs_external_ids_new()) }
    }

    /// ## `key`
    /// the external-id to lookup
    ///
    /// # Returns
    ///
    /// the value associated with `key` or [`None`] if no such
    ///  value exists.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_get_data")]
    #[doc(alias = "get_data")]
    pub fn data(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_external_ids_get_data(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// a
    ///  [`None`]-terminated array containing each key from the table.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_get_data_keys")]
    #[doc(alias = "get_data_keys")]
    pub fn data_keys(&self) -> Vec<glib::GString> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::nm_setting_ovs_external_ids_get_data_keys(
                    self.to_glib_none().0,
                    out_len.as_mut_ptr(),
                ),
                out_len.assume_init() as _,
            );
            ret
        }
    }

    /// ## `key`
    /// the key to set
    /// ## `val`
    /// the value to set or [`None`] to clear a key.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_set_data")]
    pub fn set_data(&self, key: &str, val: Option<&str>) {
        unsafe {
            ffi::nm_setting_ovs_external_ids_set_data(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.to_glib_none().0,
            );
        }
    }

    /// Checks whether `key` is a valid key for OVS' external-ids.
    /// This means, the key cannot be [`None`], not too large and valid ASCII.
    /// Also, only digits and numbers are allowed with a few special
    /// characters. They key must also not start with "NM.".
    /// ## `key`
    /// the key to check
    ///
    /// # Returns
    ///
    /// [`true`] if `key` is a valid user data key.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_check_key")]
    pub fn check_key(key: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::nm_setting_ovs_external_ids_check_key(key.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Checks whether `val` is a valid user data value. This means,
    /// value is not [`None`], not too large and valid UTF-8.
    /// ## `val`
    /// the value to check
    ///
    /// # Returns
    ///
    /// [`true`] if `val` is a valid user data value.
    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "nm_setting_ovs_external_ids_check_val")]
    pub fn check_val(val: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::nm_setting_ovs_external_ids_check_val(val.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
    #[doc(alias = "data")]
    pub fn connect_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_data_trampoline<F: Fn(&SettingOvsExternalIDs) + 'static>(
            this: *mut ffi::NMSettingOvsExternalIDs,
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
                b"notify::data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_30")))]
impl Default for SettingOvsExternalIDs {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsExternalIDs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingOvsExternalIDs")
    }
}
