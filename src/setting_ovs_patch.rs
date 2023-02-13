// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::fmt;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    /// OvsPatch Link Settings
    ///
    /// ## Properties
    ///
    ///
    /// #### `peer`
    ///  Specifies the name of the interface for the other side of the patch.
    /// The patch on the other side must also set this interface as peer.
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
    #[doc(alias = "NMSettingOvsPatch")]
    pub struct SettingOvsPatch(Object<ffi::NMSettingOvsPatch, ffi::NMSettingOvsPatchClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_ovs_patch_get_type(),
    }
}

impl SettingOvsPatch {
    /// Creates a new [`SettingOvsPatch`][crate::SettingOvsPatch] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingOvsPatch`][crate::SettingOvsPatch] object
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_patch_new")]
    pub fn new() -> SettingOvsPatch {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ovs_patch_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the [`peer`][struct@crate::SettingOvsPatch#peer] property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_patch_get_peer")]
    #[doc(alias = "get_peer")]
    pub fn peer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_ovs_patch_get_peer(self.to_glib_none().0)) }
    }

    /// Specifies the name of the interface for the other side of the patch.
    /// The patch on the other side must also set this interface as peer.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn set_peer(&self, peer: Option<&str>) {
        glib::ObjectExt::set_property(self, "peer", &peer)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "peer")]
    pub fn connect_peer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_trampoline<F: Fn(&SettingOvsPatch) + 'static>(
            this: *mut ffi::NMSettingOvsPatch,
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
                b"notify::peer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
impl Default for SettingOvsPatch {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingOvsPatch")
    }
}
