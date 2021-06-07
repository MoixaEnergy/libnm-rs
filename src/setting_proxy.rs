// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::SettingProxyMethod;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingProxy")]
    pub struct SettingProxy(Object<ffi::NMSettingProxy, ffi::NMSettingProxyClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_proxy_get_type(),
    }
}

impl SettingProxy {
    /// Creates a new [`SettingProxy`][crate::SettingProxy] object.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingProxy`][crate::SettingProxy] object
    #[doc(alias = "nm_setting_proxy_new")]
    pub fn new() -> SettingProxy {
        unsafe { Setting::from_glib_full(ffi::nm_setting_proxy_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// [`true`] if this proxy configuration is only for browser
    /// clients/schemes, [`false`] otherwise.
    #[doc(alias = "nm_setting_proxy_get_browser_only")]
    #[doc(alias = "get_browser_only")]
    pub fn is_browser_only(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_proxy_get_browser_only(
                self.to_glib_none().0,
            ))
        }
    }

    /// Returns the proxy configuration method. By default the value is [`SettingProxyMethod::None`][crate::SettingProxyMethod::None].
    /// [`SettingProxyMethod::None`][crate::SettingProxyMethod::None] should be selected for a connection intended for direct network
    /// access.
    ///
    /// # Returns
    ///
    /// the proxy configuration method
    #[doc(alias = "nm_setting_proxy_get_method")]
    #[doc(alias = "get_method")]
    pub fn method(&self) -> SettingProxyMethod {
        unsafe { from_glib(ffi::nm_setting_proxy_get_method(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the PAC script
    #[doc(alias = "nm_setting_proxy_get_pac_script")]
    #[doc(alias = "get_pac_script")]
    pub fn pac_script(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_proxy_get_pac_script(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the PAC URL for obtaining PAC file
    #[doc(alias = "nm_setting_proxy_get_pac_url")]
    #[doc(alias = "get_pac_url")]
    pub fn pac_url(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_proxy_get_pac_url(self.to_glib_none().0)) }
    }

    /// Whether the proxy configuration is for browser only.
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "browser-only")]
    pub fn set_browser_only(&self, browser_only: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"browser-only\0".as_ptr() as *const _,
                browser_only.to_value().to_glib_none().0,
            );
        }
    }

    /// Method for proxy configuration, Default is [`SettingProxyMethod::None`][crate::SettingProxyMethod::None]
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    pub fn set_method(&self, method: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"method\0".as_ptr() as *const _,
                method.to_value().to_glib_none().0,
            );
        }
    }

    /// PAC script for the connection.
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "pac-script")]
    pub fn set_pac_script(&self, pac_script: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pac-script\0".as_ptr() as *const _,
                pac_script.to_value().to_glib_none().0,
            );
        }
    }

    /// PAC URL for obtaining PAC file.
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "pac-url")]
    pub fn set_pac_url(&self, pac_url: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pac-url\0".as_ptr() as *const _,
                pac_url.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "browser-only")]
    pub fn connect_browser_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_browser_only_trampoline<F: Fn(&SettingProxy) + 'static>(
            this: *mut ffi::NMSettingProxy,
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
                b"notify::browser-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_browser_only_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "method")]
    pub fn connect_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_method_trampoline<F: Fn(&SettingProxy) + 'static>(
            this: *mut ffi::NMSettingProxy,
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
                b"notify::method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_method_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "pac-script")]
    pub fn connect_pac_script_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pac_script_trampoline<F: Fn(&SettingProxy) + 'static>(
            this: *mut ffi::NMSettingProxy,
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
                b"notify::pac-script\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pac_script_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "pac-url")]
    pub fn connect_pac_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pac_url_trampoline<F: Fn(&SettingProxy) + 'static>(
            this: *mut ffi::NMSettingProxy,
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
                b"notify::pac-url\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pac_url_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
impl Default for SettingProxy {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingProxy")
    }
}
