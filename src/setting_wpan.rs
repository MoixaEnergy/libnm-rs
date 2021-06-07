// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingWpan")]
    pub struct SettingWpan(Object<ffi::NMSettingWpan, ffi::NMSettingWpanClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_wpan_get_type(),
    }
}

impl SettingWpan {
    /// Creates a new [`SettingWpan`][crate::SettingWpan] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingWpan`][crate::SettingWpan] object
    #[doc(alias = "nm_setting_wpan_new")]
    pub fn new() -> SettingWpan {
        unsafe { Setting::from_glib_full(ffi::nm_setting_wpan_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingWpan::channel` property of the setting
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "nm_setting_wpan_get_channel")]
    #[doc(alias = "get_channel")]
    pub fn channel(&self) -> i16 {
        unsafe { ffi::nm_setting_wpan_get_channel(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingWpan::mac-address` property of the setting
    #[doc(alias = "nm_setting_wpan_get_mac_address")]
    #[doc(alias = "get_mac_address")]
    pub fn mac_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_wpan_get_mac_address(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingWpan::page` property of the setting
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "nm_setting_wpan_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self) -> i16 {
        unsafe { ffi::nm_setting_wpan_get_page(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingWpan::pan-id` property of the setting
    #[doc(alias = "nm_setting_wpan_get_pan_id")]
    #[doc(alias = "get_pan_id")]
    pub fn pan_id(&self) -> u16 {
        unsafe { ffi::nm_setting_wpan_get_pan_id(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingWpan::short-address` property of the setting
    #[doc(alias = "nm_setting_wpan_get_short_address")]
    #[doc(alias = "get_short_address")]
    pub fn short_address(&self) -> u16 {
        unsafe { ffi::nm_setting_wpan_get_short_address(self.to_glib_none().0) }
    }

    /// IEEE 802.15.4 channel. A positive integer or -1, meaning "do not
    /// set, use whatever the device is already set to".
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_channel(&self, channel: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"channel\0".as_ptr() as *const _,
                channel.to_value().to_glib_none().0,
            );
        }
    }

    /// If specified, this connection will only apply to the IEEE 802.15.4 (WPAN)
    /// MAC layer device whose permanent MAC address matches.
    #[doc(alias = "mac-address")]
    pub fn get_property_mac_address(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mac-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mac-address` getter")
        }
    }

    /// If specified, this connection will only apply to the IEEE 802.15.4 (WPAN)
    /// MAC layer device whose permanent MAC address matches.
    #[doc(alias = "mac-address")]
    pub fn set_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mac-address\0".as_ptr() as *const _,
                mac_address.to_value().to_glib_none().0,
            );
        }
    }

    /// IEEE 802.15.4 channel page. A positive integer or -1, meaning "do not
    /// set, use whatever the device is already set to".
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_page(&self, page: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"page\0".as_ptr() as *const _,
                page.to_value().to_glib_none().0,
            );
        }
    }

    /// IEEE 802.15.4 Personal Area Network (PAN) identifier.
    #[doc(alias = "pan-id")]
    pub fn get_property_pan_id(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pan-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pan-id` getter")
        }
    }

    /// IEEE 802.15.4 Personal Area Network (PAN) identifier.
    #[doc(alias = "pan-id")]
    pub fn set_pan_id(&self, pan_id: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pan-id\0".as_ptr() as *const _,
                pan_id.to_value().to_glib_none().0,
            );
        }
    }

    /// Short IEEE 802.15.4 address to be used within a restricted environment.
    #[doc(alias = "short-address")]
    pub fn get_property_short_address(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"short-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `short-address` getter")
        }
    }

    /// Short IEEE 802.15.4 address to be used within a restricted environment.
    #[doc(alias = "short-address")]
    pub fn set_short_address(&self, short_address: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"short-address\0".as_ptr() as *const _,
                short_address.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "channel")]
    pub fn connect_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_channel_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut ffi::NMSettingWpan,
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
                b"notify::channel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_channel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mac-address")]
    pub fn connect_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut ffi::NMSettingWpan,
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
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "page")]
    pub fn connect_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut ffi::NMSettingWpan,
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
                b"notify::page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pan-id")]
    pub fn connect_pan_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pan_id_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut ffi::NMSettingWpan,
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
                b"notify::pan-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pan_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "short-address")]
    pub fn connect_short_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_short_address_trampoline<F: Fn(&SettingWpan) + 'static>(
            this: *mut ffi::NMSettingWpan,
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
                b"notify::short-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_short_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
impl Default for SettingWpan {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingWpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingWpan")
    }
}
