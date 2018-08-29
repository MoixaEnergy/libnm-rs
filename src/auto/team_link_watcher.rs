// This file was generated by gir (https://github.com/gtk-rs/gir @ 609779c)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use Error;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use TeamLinkWatcherArpPingFlags;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TeamLinkWatcher(Shared<ffi::NMTeamLinkWatcher>);

    match fn {
        ref => |ptr| ffi::nm_team_link_watcher_ref(ptr),
        unref => |ptr| ffi::nm_team_link_watcher_unref(ptr),
        get_type => || ffi::nm_team_link_watcher_get_type(),
    }
}

impl TeamLinkWatcher {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new_arp_ping(
        init_wait: i32,
        interval: i32,
        missed_max: i32,
        target_host: &str,
        source_host: &str,
        flags: TeamLinkWatcherArpPingFlags,
    ) -> Result<TeamLinkWatcher, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_team_link_watcher_new_arp_ping(
                init_wait,
                interval,
                missed_max,
                target_host.to_glib_none().0,
                source_host.to_glib_none().0,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new_ethtool(delay_up: i32, delay_down: i32) -> Result<TeamLinkWatcher, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_team_link_watcher_new_ethtool(delay_up, delay_down, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new_nsna_ping(
        init_wait: i32,
        interval: i32,
        missed_max: i32,
        target_host: &str,
    ) -> Result<TeamLinkWatcher, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_team_link_watcher_new_nsna_ping(
                init_wait,
                interval,
                missed_max,
                target_host.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn dup(&self) -> Option<TeamLinkWatcher> {
        unsafe { from_glib_full(ffi::nm_team_link_watcher_dup(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn equal(&self, other: &TeamLinkWatcher) -> bool {
        unsafe {
            from_glib(ffi::nm_team_link_watcher_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_delay_down(&self) -> i32 {
        unsafe { ffi::nm_team_link_watcher_get_delay_down(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_delay_up(&self) -> i32 {
        unsafe { ffi::nm_team_link_watcher_get_delay_up(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_flags(&self) -> TeamLinkWatcherArpPingFlags {
        unsafe { from_glib(ffi::nm_team_link_watcher_get_flags(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_init_wait(&self) -> i32 {
        unsafe { ffi::nm_team_link_watcher_get_init_wait(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_interval(&self) -> i32 {
        unsafe { ffi::nm_team_link_watcher_get_interval(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_missed_max(&self) -> i32 {
        unsafe { ffi::nm_team_link_watcher_get_missed_max(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_team_link_watcher_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_source_host(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_team_link_watcher_get_source_host(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_target_host(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_team_link_watcher_get_target_host(
                self.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for TeamLinkWatcher {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TeamLinkWatcher {}
