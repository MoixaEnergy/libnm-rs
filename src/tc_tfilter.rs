// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
use crate::TCAction;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::GString;
use nm_sys;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCTfilter(Shared<nm_sys::NMTCTfilter>);

    match fn {
        ref => |ptr| nm_sys::nm_tc_tfilter_ref(ptr),
        unref => |ptr| nm_sys::nm_tc_tfilter_unref(ptr),
        get_type => || nm_sys::nm_tc_tfilter_get_type(),
    }
}

impl TCTfilter {
    /// Creates a new `TCTfilter` object.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `kind`
    /// name of the queueing discipline
    /// ## `parent`
    /// the parent queueing discipline
    ///
    /// # Returns
    ///
    /// the new `TCTfilter` object, or `None` on error
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new(kind: &str, parent: u32) -> Result<TCTfilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = nm_sys::nm_tc_tfilter_new(kind.to_glib_none().0, parent, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Creates a copy of `self`
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn dup(&self) -> Option<TCTfilter> {
        unsafe { from_glib_full(nm_sys::nm_tc_tfilter_dup(self.to_glib_none().0)) }
    }

    /// Determines if two `TCTfilter` objects contain the same kind, family,
    /// handle, parent and info.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `other`
    /// the `TCTfilter` to compare `self` to.
    ///
    /// # Returns
    ///
    /// `true` if the objects contain the same values, `false` if they do not.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn equal(&self, other: &TCTfilter) -> bool {
        unsafe {
            from_glib(nm_sys::nm_tc_tfilter_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the action associated with a traffic filter.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_action(&self) -> Option<TCAction> {
        unsafe { from_glib_full(nm_sys::nm_tc_tfilter_get_action(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_handle(&self) -> u32 {
        unsafe { nm_sys::nm_tc_tfilter_get_handle(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_12`
    ///
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_kind(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_tc_tfilter_get_kind(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the parent class
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_parent(&self) -> u32 {
        unsafe { nm_sys::nm_tc_tfilter_get_parent(self.to_glib_none().0) }
    }

    /// Sets the action associated with a traffic filter.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `action`
    /// the action object
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_action(&self, action: &TCAction) {
        unsafe {
            nm_sys::nm_tc_tfilter_set_action(self.to_glib_none().0, action.to_glib_none().0);
        }
    }

    /// Sets the queueing discipline handle.
    ///
    /// Feature: `v1_12`
    ///
    /// ## `handle`
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_handle(&self, handle: u32) {
        unsafe {
            nm_sys::nm_tc_tfilter_set_handle(self.to_glib_none().0, handle);
        }
    }
}

impl PartialEq for TCTfilter {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCTfilter {}
