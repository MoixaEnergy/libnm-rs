// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::SettingTunMode;
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
    #[doc(alias = "NMSettingTun")]
    pub struct SettingTun(Object<ffi::NMSettingTun, ffi::NMSettingTunClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_tun_get_type(),
    }
}

impl SettingTun {
    /// Creates a new [`SettingTun`][crate::SettingTun] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingTun`][crate::SettingTun] object
    #[doc(alias = "nm_setting_tun_new")]
    pub fn new() -> SettingTun {
        unsafe { Setting::from_glib_full(ffi::nm_setting_tun_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::group` property of the setting
    #[doc(alias = "nm_setting_tun_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_tun_get_group(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::mode` property of the setting
    #[doc(alias = "nm_setting_tun_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> SettingTunMode {
        unsafe { from_glib(ffi::nm_setting_tun_get_mode(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::multi-queue` property of the setting
    #[doc(alias = "nm_setting_tun_get_multi_queue")]
    #[doc(alias = "get_multi_queue")]
    pub fn is_multi_queue(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_tun_get_multi_queue(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::owner` property of the setting
    #[doc(alias = "nm_setting_tun_get_owner")]
    #[doc(alias = "get_owner")]
    pub fn owner(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_tun_get_owner(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::pi` property of the setting
    #[doc(alias = "nm_setting_tun_get_pi")]
    #[doc(alias = "get_pi")]
    pub fn is_pi(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_tun_get_pi(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTun::vnet_hdr` property of the setting
    #[doc(alias = "nm_setting_tun_get_vnet_hdr")]
    #[doc(alias = "get_vnet_hdr")]
    pub fn is_vnet_hdr(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_tun_get_vnet_hdr(self.to_glib_none().0)) }
    }

    /// The group ID which will own the device. If set to [`None`] everyone
    /// will be able to use the device.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn set_group(&self, group: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"group\0".as_ptr() as *const _,
                group.to_value().to_glib_none().0,
            );
        }
    }

    /// The operating mode of the virtual device. Allowed values are
    /// [`SettingTunMode::Tun`][crate::SettingTunMode::Tun] to create a layer 3 device and
    /// [`SettingTunMode::Tap`][crate::SettingTunMode::Tap] to create an Ethernet-like layer 2
    /// one.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn set_mode(&self, mode: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mode\0".as_ptr() as *const _,
                mode.to_value().to_glib_none().0,
            );
        }
    }

    /// If the property is set to [`true`], the interface will support
    /// multiple file descriptors (queues) to parallelize packet
    /// sending or receiving. Otherwise, the interface will only
    /// support a single queue.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "multi-queue")]
    pub fn set_multi_queue(&self, multi_queue: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"multi-queue\0".as_ptr() as *const _,
                multi_queue.to_value().to_glib_none().0,
            );
        }
    }

    /// The user ID which will own the device. If set to [`None`] everyone
    /// will be able to use the device.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn set_owner(&self, owner: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"owner\0".as_ptr() as *const _,
                owner.to_value().to_glib_none().0,
            );
        }
    }

    /// If [`true`] the interface will prepend a 4 byte header describing the
    /// physical interface to the packets.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn set_pi(&self, pi: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pi\0".as_ptr() as *const _,
                pi.to_value().to_glib_none().0,
            );
        }
    }

    /// If [`true`] the IFF_VNET_HDR the tunnel packets will include a virtio
    /// network header.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "vnet-hdr")]
    pub fn set_vnet_hdr(&self, vnet_hdr: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"vnet-hdr\0".as_ptr() as *const _,
                vnet_hdr.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "group")]
    pub fn connect_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "multi-queue")]
    pub fn connect_multi_queue_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_multi_queue_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::multi-queue\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multi_queue_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "owner")]
    pub fn connect_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "pi")]
    pub fn connect_pi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pi_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::pi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "vnet-hdr")]
    pub fn connect_vnet_hdr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vnet_hdr_trampoline<F: Fn(&SettingTun) + 'static>(
            this: *mut ffi::NMSettingTun,
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
                b"notify::vnet-hdr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vnet_hdr_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
impl Default for SettingTun {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingTun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingTun")
    }
}
