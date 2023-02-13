// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    /// 6LoWPAN Settings
    ///
    /// ## Properties
    ///
    ///
    /// #### `parent`
    ///  If given, specifies the parent interface name or parent connection UUID
    /// from which this 6LowPAN interface should be created.
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
    #[doc(alias = "NMSetting6Lowpan")]
    pub struct Setting6Lowpan(Object<ffi::NMSetting6Lowpan, ffi::NMSetting6LowpanClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_6lowpan_get_type(),
    }
}

impl Setting6Lowpan {
    /// Creates a new [`Setting6Lowpan`][crate::Setting6Lowpan] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`Setting6Lowpan`][crate::Setting6Lowpan] object
    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "nm_setting_6lowpan_new")]
    pub fn new() -> Setting6Lowpan {
        unsafe { Setting::from_glib_full(ffi::nm_setting_6lowpan_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the [`parent`][struct@crate::Setting6Lowpan#parent] property of the setting
    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "nm_setting_6lowpan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_6lowpan_get_parent(self.to_glib_none().0)) }
    }

    /// If given, specifies the parent interface name or parent connection UUID
    /// from which this 6LowPAN interface should be created.
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub fn get_property_parent(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "parent")
    }

    /// If given, specifies the parent interface name or parent connection UUID
    /// from which this 6LowPAN interface should be created.
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub fn set_parent(&self, parent: Option<&str>) {
        glib::ObjectExt::set_property(self, "parent", &parent)
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&Setting6Lowpan) + 'static>(
            this: *mut ffi::NMSetting6Lowpan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
impl Default for Setting6Lowpan {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Setting6Lowpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Setting6Lowpan")
    }
}
