// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::TeamLinkWatcher;
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
    #[doc(alias = "NMSettingTeamPort")]
    pub struct SettingTeamPort(Object<ffi::NMSettingTeamPort, ffi::NMSettingTeamPortClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_team_port_get_type(),
    }
}

impl SettingTeamPort {
    /// Creates a new [`SettingTeamPort`][crate::SettingTeamPort] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingTeamPort`][crate::SettingTeamPort] object
    #[doc(alias = "nm_setting_team_port_new")]
    pub fn new() -> SettingTeamPort {
        unsafe { Setting::from_glib_full(ffi::nm_setting_team_port_new()).unsafe_cast() }
    }

    /// Appends a new link watcher to the setting.
    /// ## `link_watcher`
    /// the link watcher to add
    ///
    /// # Returns
    ///
    /// [`true`] if the link watcher is added; [`false`] if an identical link
    /// watcher was already there.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_add_link_watcher")]
    pub fn add_link_watcher(&self, link_watcher: &TeamLinkWatcher) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_port_add_link_watcher(
                self.to_glib_none().0,
                link_watcher.to_glib_none().0,
            ))
        }
    }

    /// Removes all configured link watchers.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_clear_link_watchers")]
    pub fn clear_link_watchers(&self) {
        unsafe {
            ffi::nm_setting_team_port_clear_link_watchers(self.to_glib_none().0);
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::config` property of the setting
    #[doc(alias = "nm_setting_team_port_get_config")]
    #[doc(alias = "get_config")]
    pub fn config(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_team_port_get_config(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::lacp-key` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_lacp_key")]
    #[doc(alias = "get_lacp_key")]
    pub fn lacp_key(&self) -> i32 {
        unsafe { ffi::nm_setting_team_port_get_lacp_key(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::lacp-prio` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_lacp_prio")]
    #[doc(alias = "get_lacp_prio")]
    pub fn lacp_prio(&self) -> i32 {
        unsafe { ffi::nm_setting_team_port_get_lacp_prio(self.to_glib_none().0) }
    }

    /// ## `idx`
    /// index number of the link watcher to return
    ///
    /// # Returns
    ///
    /// the link watcher at index `idx`.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_link_watcher")]
    #[doc(alias = "get_link_watcher")]
    pub fn link_watcher(&self, idx: u32) -> Option<TeamLinkWatcher> {
        unsafe {
            from_glib_none(ffi::nm_setting_team_port_get_link_watcher(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the number of configured link watchers
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_num_link_watchers")]
    #[doc(alias = "get_num_link_watchers")]
    pub fn num_link_watchers(&self) -> u32 {
        unsafe { ffi::nm_setting_team_port_get_num_link_watchers(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::prio` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_prio")]
    #[doc(alias = "get_prio")]
    pub fn prio(&self) -> i32 {
        unsafe { ffi::nm_setting_team_port_get_prio(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::queue_id` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_queue_id")]
    #[doc(alias = "get_queue_id")]
    pub fn queue_id(&self) -> i32 {
        unsafe { ffi::nm_setting_team_port_get_queue_id(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingTeamPort::sticky` property of the setting
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_get_sticky")]
    #[doc(alias = "get_sticky")]
    pub fn is_sticky(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_team_port_get_sticky(self.to_glib_none().0)) }
    }

    /// Removes the link watcher at index `idx`.
    /// ## `idx`
    /// index number of the link watcher to remove
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_remove_link_watcher")]
    pub fn remove_link_watcher(&self, idx: u32) {
        unsafe {
            ffi::nm_setting_team_port_remove_link_watcher(self.to_glib_none().0, idx);
        }
    }

    /// Removes the link watcher entry matching link_watcher.
    /// ## `link_watcher`
    /// the link watcher to remove
    ///
    /// # Returns
    ///
    /// [`true`] if the link watcher was found and removed, [`false`] otherwise.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_team_port_remove_link_watcher_by_value")]
    pub fn remove_link_watcher_by_value(&self, link_watcher: &TeamLinkWatcher) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_team_port_remove_link_watcher_by_value(
                self.to_glib_none().0,
                link_watcher.to_glib_none().0,
            ))
        }
    }

    /// The JSON configuration for the team port. The property should contain raw
    /// JSON configuration data suitable for teamd, because the value is passed
    /// directly to teamd. If not specified, the default configuration is
    /// used. See man teamd.conf for the format details.
    pub fn set_config(&self, config: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"config\0".as_ptr() as *const _,
                config.to_value().to_glib_none().0,
            );
        }
    }

    /// Corresponds to the teamd ports.PORTIFNAME.lacp_key.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "lacp-key")]
    pub fn set_lacp_key(&self, lacp_key: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"lacp-key\0".as_ptr() as *const _,
                lacp_key.to_value().to_glib_none().0,
            );
        }
    }

    /// Corresponds to the teamd ports.PORTIFNAME.lacp_prio.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "lacp-prio")]
    pub fn set_lacp_prio(&self, lacp_prio: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"lacp-prio\0".as_ptr() as *const _,
                lacp_prio.to_value().to_glib_none().0,
            );
        }
    }

    /// Corresponds to the teamd ports.PORTIFNAME.prio.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    pub fn set_prio(&self, prio: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"prio\0".as_ptr() as *const _,
                prio.to_value().to_glib_none().0,
            );
        }
    }

    /// Corresponds to the teamd ports.PORTIFNAME.queue_id.
    /// When set to -1 means the parameter is skipped from the json config.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "queue-id")]
    pub fn set_queue_id(&self, queue_id: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"queue-id\0".as_ptr() as *const _,
                queue_id.to_value().to_glib_none().0,
            );
        }
    }

    /// Corresponds to the teamd ports.PORTIFNAME.sticky.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    pub fn set_sticky(&self, sticky: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"sticky\0".as_ptr() as *const _,
                sticky.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "config")]
    pub fn connect_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_config_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_config_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "lacp-key")]
    pub fn connect_lacp_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lacp_key_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::lacp-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lacp_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "lacp-prio")]
    pub fn connect_lacp_prio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lacp_prio_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::lacp-prio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lacp_prio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "prio")]
    pub fn connect_prio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_prio_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::prio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_prio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "queue-id")]
    pub fn connect_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::queue-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_queue_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "sticky")]
    pub fn connect_sticky_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sticky_trampoline<F: Fn(&SettingTeamPort) + 'static>(
            this: *mut ffi::NMSettingTeamPort,
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
                b"notify::sticky\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sticky_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingTeamPort {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingTeamPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingTeamPort")
    }
}
