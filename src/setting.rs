// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::{Connection, SettingCompareFlags, SettingSecretFlags};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v1_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
use std::mem;
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    ///
    ///
    /// This is an Abstract Base Class, you cannot instantiate it.
    ///
    /// ## Properties
    ///
    ///
    /// #### `name`
    ///  The setting's name, which uniquely identifies the setting within the
    /// connection. Each setting type has a name unique to that type, for
    /// example "ppp" or "802-11-wireless" or "802-3-ethernet".
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`SettingExt`][trait@crate::prelude::SettingExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMSetting")]
    pub struct Setting(Object<ffi::NMSetting, ffi::NMSettingClass>);

    match fn {
        type_ => || ffi::nm_setting_get_type(),
    }
}

impl Setting {
    pub const NONE: Option<&'static Setting> = None;

    /// Returns the `GType` of the setting's class for a given setting name.
    /// ## `name`
    /// a setting name
    ///
    /// # Returns
    ///
    /// the `GType` of the setting's class, or `G_TYPE_INVALID` if
    ///  `name` is not recognized.
    #[doc(alias = "nm_setting_lookup_type")]
    pub fn lookup_type(name: &str) -> glib::types::Type {
        unsafe { from_glib(ffi::nm_setting_lookup_type(name.to_glib_none().0)) }
    }
}

impl fmt::Display for Setting {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&SettingExt::to_str(self))
    }
}

/// Trait containing all [`struct@Setting`] methods.
///
/// # Implementors
///
/// [`Setting6Lowpan`][struct@crate::Setting6Lowpan], [`Setting8021x`][struct@crate::Setting8021x], [`SettingAdsl`][struct@crate::SettingAdsl], [`SettingBluetooth`][struct@crate::SettingBluetooth], [`SettingBondPort`][struct@crate::SettingBondPort], [`SettingBond`][struct@crate::SettingBond], [`SettingBridgePort`][struct@crate::SettingBridgePort], [`SettingBridge`][struct@crate::SettingBridge], [`SettingCdma`][struct@crate::SettingCdma], [`SettingConnection`][struct@crate::SettingConnection], [`SettingDcb`][struct@crate::SettingDcb], [`SettingDummy`][struct@crate::SettingDummy], [`SettingEthtool`][struct@crate::SettingEthtool], [`SettingGeneric`][struct@crate::SettingGeneric], [`SettingGsm`][struct@crate::SettingGsm], [`SettingHostname`][struct@crate::SettingHostname], [`SettingIPConfig`][struct@crate::SettingIPConfig], [`SettingIPTunnel`][struct@crate::SettingIPTunnel], [`SettingInfiniband`][struct@crate::SettingInfiniband], [`SettingMacsec`][struct@crate::SettingMacsec], [`SettingMacvlan`][struct@crate::SettingMacvlan], [`SettingMatch`][struct@crate::SettingMatch], [`SettingOlpcMesh`][struct@crate::SettingOlpcMesh], [`SettingOvsBridge`][struct@crate::SettingOvsBridge], [`SettingOvsDpdk`][struct@crate::SettingOvsDpdk], [`SettingOvsExternalIDs`][struct@crate::SettingOvsExternalIDs], [`SettingOvsInterface`][struct@crate::SettingOvsInterface], [`SettingOvsPatch`][struct@crate::SettingOvsPatch], [`SettingOvsPort`][struct@crate::SettingOvsPort], [`SettingPpp`][struct@crate::SettingPpp], [`SettingPppoe`][struct@crate::SettingPppoe], [`SettingProxy`][struct@crate::SettingProxy], [`SettingSerial`][struct@crate::SettingSerial], [`SettingSriov`][struct@crate::SettingSriov], [`SettingTCConfig`][struct@crate::SettingTCConfig], [`SettingTeamPort`][struct@crate::SettingTeamPort], [`SettingTeam`][struct@crate::SettingTeam], [`SettingTun`][struct@crate::SettingTun], [`SettingUser`][struct@crate::SettingUser], [`SettingVeth`][struct@crate::SettingVeth], [`SettingVlan`][struct@crate::SettingVlan], [`SettingVpn`][struct@crate::SettingVpn], [`SettingVrf`][struct@crate::SettingVrf], [`SettingVxlan`][struct@crate::SettingVxlan], [`SettingWifiP2P`][struct@crate::SettingWifiP2P], [`SettingWimax`][struct@crate::SettingWimax], [`SettingWireGuard`][struct@crate::SettingWireGuard], [`SettingWired`][struct@crate::SettingWired], [`SettingWirelessSecurity`][struct@crate::SettingWirelessSecurity], [`SettingWireless`][struct@crate::SettingWireless], [`SettingWpan`][struct@crate::SettingWpan], [`Setting`][struct@crate::Setting]
pub trait SettingExt: 'static {
    #[doc(alias = "nm_setting_compare")]
    fn compare(&self, b: &impl IsA<Setting>, flags: SettingCompareFlags) -> bool;

    //#[doc(alias = "nm_setting_diff")]
    //fn diff(&self, b: &impl IsA<Setting>, flags: SettingCompareFlags, invert_results: bool, results: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 7 }) -> bool;

    /// Duplicates a [`Setting`][crate::Setting].
    ///
    /// # Returns
    ///
    /// a new [`Setting`][crate::Setting] containing the same properties and values as the
    /// source [`Setting`][crate::Setting]
    #[doc(alias = "nm_setting_duplicate")]
    #[must_use]
    fn duplicate(&self) -> Option<Setting>;

    //#[doc(alias = "nm_setting_enumerate_values")]
    //fn enumerate_values(&self, func: /*Unimplemented*/FnMut(&Setting, &str, /*Ignored*/glib::Value, /*Ignored*/glib::ParamFlags), user_data: /*Unimplemented*/Option<Basic: Pointer>);

    /// Gets the D-Bus marshalling type of a property. `property_name` is a D-Bus
    /// property name, which may not necessarily be a [`glib::Object`][crate::glib::Object] property.
    /// ## `property_name`
    /// the property of `self` to get the type of
    ///
    /// # Returns
    ///
    /// the D-Bus marshalling type of `property` on `self`.
    #[doc(alias = "nm_setting_get_dbus_property_type")]
    #[doc(alias = "get_dbus_property_type")]
    fn dbus_property_type(&self, property_name: &str) -> Option<glib::VariantType>;

    /// Returns the type name of the [`Setting`][crate::Setting] object
    ///
    /// # Returns
    ///
    /// a string containing the type name of the [`Setting`][crate::Setting] object,
    /// like 'ppp' or 'wireless' or 'wired'.
    #[doc(alias = "nm_setting_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    //#[cfg(any(feature = "v1_26", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    //#[doc(alias = "nm_setting_option_clear_by_name")]
    //fn option_clear_by_name(&self, predicate: Option<&mut dyn (FnMut() -> bool)>);

    /// ## `opt_name`
    /// the option name to request.
    ///
    /// # Returns
    ///
    /// the [`glib::Variant`][struct@crate::glib::Variant] or [`None`] if the option
    ///  is not set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_get")]
    fn option_get(&self, opt_name: &str) -> Option<glib::Variant>;

    /// Gives the name of all set options.
    ///
    /// # Returns
    ///
    ///
    ///  A [`None`] terminated array of key names. If no names are present, this returns
    ///  [`None`]. The returned array and the names are owned by `NMSetting` and might be invalidated
    ///  by the next operation.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_get_all_names")]
    fn option_get_all_names(&self) -> Vec<glib::GString>;

    /// ## `opt_name`
    /// the option to get
    ///
    /// # Returns
    ///
    /// [`true`] if `opt_name` is set to a boolean variant.
    ///
    /// ## `out_value`
    /// the optional output value.
    ///  If the option is unset, [`false`] will be returned.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_get_boolean")]
    fn option_get_boolean(&self, opt_name: &str) -> Option<bool>;

    /// ## `opt_name`
    /// the option to get
    ///
    /// # Returns
    ///
    /// [`true`] if `opt_name` is set to a uint32 variant.
    ///
    /// ## `out_value`
    /// the optional output value.
    ///  If the option is unset, 0 will be returned.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_get_uint32")]
    fn option_get_uint32(&self, opt_name: &str) -> Option<u32>;

    /// If `variant` is [`None`], this clears the option if it is set.
    /// Otherwise, `variant` is set as the option. If `variant` is
    /// a floating reference, it will be consumed.
    ///
    /// Note that not all setting types support options. It is a bug
    /// setting a variant to a setting that doesn't support it.
    /// Currently, only [`SettingEthtool`][crate::SettingEthtool] supports it.
    /// ## `opt_name`
    /// the option name to set
    /// ## `variant`
    /// the variant to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_set")]
    fn option_set(&self, opt_name: &str, variant: Option<&glib::Variant>);

    /// Like [`option_set()`][Self::option_set()] to set a boolean GVariant.
    /// ## `value`
    /// the value to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_set_boolean")]
    fn option_set_boolean(&self, opt_name: &str, value: bool);

    /// Like [`option_set()`][Self::option_set()] to set a uint32 GVariant.
    /// ## `value`
    /// the value to set.
    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    #[doc(alias = "nm_setting_option_set_uint32")]
    fn option_set_uint32(&self, opt_name: &str, value: u32);

    /// For a given secret, stores the [`SettingSecretFlags`][crate::SettingSecretFlags] describing how to
    /// handle that secret.
    /// ## `secret_name`
    /// the secret key name to set flags for
    /// ## `flags`
    /// the [`SettingSecretFlags`][crate::SettingSecretFlags] for the secret
    ///
    /// # Returns
    ///
    /// [`true`] on success (if the given secret name was a valid property of
    /// this setting, and if that property is secret), [`false`] if not
    #[doc(alias = "nm_setting_set_secret_flags")]
    fn set_secret_flags(
        &self,
        secret_name: &str,
        flags: SettingSecretFlags,
    ) -> Result<(), glib::Error>;

    /// Convert the setting (including secrets!) into a string. For debugging
    /// purposes ONLY, should NOT be used for serialization of the setting,
    /// or machine-parsed in any way. The output format is not guaranteed to
    /// be stable and may change at any time.
    ///
    /// # Returns
    ///
    /// an allocated string containing a textual representation of the
    /// setting's properties and values, which the caller should
    /// free with `g_free()`
    #[doc(alias = "nm_setting_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString;

    /// Validates the setting. Each setting's properties have allowed values, and
    /// some are dependent on other values (hence the need for `connection`). The
    /// returned [`glib::Error`][crate::glib::Error] contains information about which property of the setting
    /// failed validation, and in what way that property failed validation.
    /// ## `connection`
    /// the [`Connection`][crate::Connection] that `self` came from, or
    ///  [`None`] if `self` is being verified in isolation.
    ///
    /// # Returns
    ///
    /// [`true`] if the setting is valid, [`false`] if it is not
    #[doc(alias = "nm_setting_verify")]
    fn verify(&self, connection: Option<&impl IsA<Connection>>) -> Result<(), glib::Error>;

    /// Verifies the secrets in the setting.
    /// The returned [`glib::Error`][crate::glib::Error] contains information about which secret of the setting
    /// failed validation, and in what way that secret failed validation.
    /// The secret validation is done separately from main setting validation, because
    /// in some cases connection failure is not desired just for the secrets.
    /// ## `connection`
    /// the [`Connection`][crate::Connection] that `self` came from, or
    ///  [`None`] if `self` is being verified in isolation.
    ///
    /// # Returns
    ///
    /// [`true`] if the setting secrets are valid, [`false`] if they are not
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_setting_verify_secrets")]
    fn verify_secrets(&self, connection: Option<&impl IsA<Connection>>) -> Result<(), glib::Error>;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Setting>> SettingExt for O {
    fn compare(&self, b: &impl IsA<Setting>, flags: SettingCompareFlags) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_compare(
                self.as_ref().to_glib_none().0,
                b.as_ref().to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    //fn diff(&self, b: &impl IsA<Setting>, flags: SettingCompareFlags, invert_results: bool, results: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 7 }) -> bool {
    //    unsafe { TODO: call ffi:nm_setting_diff() }
    //}

    fn duplicate(&self) -> Option<Setting> {
        unsafe { from_glib_full(ffi::nm_setting_duplicate(self.as_ref().to_glib_none().0)) }
    }

    //fn enumerate_values(&self, func: /*Unimplemented*/FnMut(&Setting, &str, /*Ignored*/glib::Value, /*Ignored*/glib::ParamFlags), user_data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:nm_setting_enumerate_values() }
    //}

    fn dbus_property_type(&self, property_name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::nm_setting_get_dbus_property_type(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_get_name(self.as_ref().to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v1_26", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    //fn option_clear_by_name(&self, predicate: Option<&mut dyn (FnMut() -> bool)>) {
    //    unsafe { TODO: call ffi:nm_setting_option_clear_by_name() }
    //}

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_get(&self, opt_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_setting_option_get(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_get_all_names(&self) -> Vec<glib::GString> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::nm_setting_option_get_all_names(
                    self.as_ref().to_glib_none().0,
                    out_len.as_mut_ptr(),
                ),
                out_len.assume_init() as _,
            );
            ret
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_get_boolean(&self, opt_name: &str) -> Option<bool> {
        unsafe {
            let mut out_value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::nm_setting_option_get_boolean(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                out_value.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(out_value.assume_init()))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_get_uint32(&self, opt_name: &str) -> Option<u32> {
        unsafe {
            let mut out_value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::nm_setting_option_get_uint32(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                out_value.as_mut_ptr(),
            ));
            if ret {
                Some(out_value.assume_init())
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_set(&self, opt_name: &str, variant: Option<&glib::Variant>) {
        unsafe {
            ffi::nm_setting_option_set(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                variant.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_set_boolean(&self, opt_name: &str, value: bool) {
        unsafe {
            ffi::nm_setting_option_set_boolean(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_26")))]
    fn option_set_uint32(&self, opt_name: &str, value: u32) {
        unsafe {
            ffi::nm_setting_option_set_uint32(
                self.as_ref().to_glib_none().0,
                opt_name.to_glib_none().0,
                value,
            );
        }
    }

    fn set_secret_flags(
        &self,
        secret_name: &str,
        flags: SettingSecretFlags,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_setting_set_secret_flags(
                self.as_ref().to_glib_none().0,
                secret_name.to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::nm_setting_to_string(self.as_ref().to_glib_none().0)) }
    }

    fn verify(&self, connection: Option<&impl IsA<Connection>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_setting_verify(
                self.as_ref().to_glib_none().0,
                connection.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn verify_secrets(&self, connection: Option<&impl IsA<Connection>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_setting_verify_secrets(
                self.as_ref().to_glib_none().0,
                connection.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Setting>, F: Fn(&P) + 'static>(
            this: *mut ffi::NMSetting,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Setting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
