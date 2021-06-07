// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "NMSettingGeneric")]
    pub struct SettingGeneric(Object<ffi::NMSettingGeneric, ffi::NMSettingGenericClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_generic_get_type(),
    }
}

impl SettingGeneric {
    /// Creates a new [`SettingGeneric`][crate::SettingGeneric] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingGeneric`][crate::SettingGeneric] object
    #[doc(alias = "nm_setting_generic_new")]
    pub fn new() -> SettingGeneric {
        unsafe { Setting::from_glib_full(ffi::nm_setting_generic_new()).unsafe_cast() }
    }
}

impl Default for SettingGeneric {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingGeneric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingGeneric")
    }
}
