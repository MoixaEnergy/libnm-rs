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

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCQdisc(Shared<ffi::NMTCQdisc>);

    match fn {
        ref => |ptr| ffi::nm_tc_qdisc_ref(ptr),
        unref => |ptr| ffi::nm_tc_qdisc_unref(ptr),
        get_type => || ffi::nm_tc_qdisc_get_type(),
    }
}

impl TCQdisc {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new(kind: &str, parent: u32) -> Result<TCQdisc, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_tc_qdisc_new(kind.to_glib_none().0, parent, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn dup(&self) -> Option<TCQdisc> {
        unsafe { from_glib_full(ffi::nm_tc_qdisc_dup(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn equal(&self, other: &TCQdisc) -> bool {
        unsafe {
            from_glib(ffi::nm_tc_qdisc_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_handle(&self) -> u32 {
        unsafe { ffi::nm_tc_qdisc_get_handle(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_kind(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_tc_qdisc_get_kind(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_parent(&self) -> u32 {
        unsafe { ffi::nm_tc_qdisc_get_parent(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_handle(&self, handle: u32) {
        unsafe {
            ffi::nm_tc_qdisc_set_handle(self.to_glib_none().0, handle);
        }
    }
}

impl PartialEq for TCQdisc {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCQdisc {}
