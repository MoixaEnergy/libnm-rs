// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct TCQdisc(Shared<ffi::NMTCQdisc>);

    match fn {
        ref => |ptr| ffi::nm_tc_qdisc_ref(ptr),
        unref => |ptr| ffi::nm_tc_qdisc_unref(ptr),
        type_ => || ffi::nm_tc_qdisc_get_type(),
    }
}

impl TCQdisc {
    /// Creates a new [`TCQdisc`][crate::TCQdisc] object.
    /// ## `kind`
    /// name of the queueing discipline
    /// ## `parent`
    /// the parent queueing discipline
    ///
    /// # Returns
    ///
    /// the new [`TCQdisc`][crate::TCQdisc] object, or [`None`] on error
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_new")]
    pub fn new(kind: &str, parent: u32) -> Result<TCQdisc, glib::Error> {
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

    /// Creates a copy of `self`
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_dup")]
    #[must_use]
    pub fn dup(&self) -> Option<TCQdisc> {
        unsafe { from_glib_full(ffi::nm_tc_qdisc_dup(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_equal")]
    fn equal(&self, other: &TCQdisc) -> bool {
        unsafe {
            from_glib(ffi::nm_tc_qdisc_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    /// ## `name`
    /// the name of an qdisc attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or [`None`] if `self` has no such attribute.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_tc_qdisc_get_attribute")]
    #[doc(alias = "get_attribute")]
    pub fn attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_tc_qdisc_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    /// Gets an array of attribute names defined on `self`.
    ///
    /// # Returns
    ///
    /// a [`None`]-terminated array of attribute names
    ///  or [`None`] if no attributes are set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_tc_qdisc_get_attribute_names")]
    #[doc(alias = "get_attribute_names")]
    pub fn attribute_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::nm_tc_qdisc_get_attribute_names(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_get_handle")]
    #[doc(alias = "get_handle")]
    pub fn handle(&self) -> u32 {
        unsafe { ffi::nm_tc_qdisc_get_handle(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_get_kind")]
    #[doc(alias = "get_kind")]
    pub fn kind(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_tc_qdisc_get_kind(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the parent class
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> u32 {
        unsafe { ffi::nm_tc_qdisc_get_parent(self.to_glib_none().0) }
    }

    /// Sets or clears the named attribute on `self` to the given value.
    /// ## `name`
    /// the name of an qdisc attribute
    /// ## `value`
    /// the value
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_tc_qdisc_set_attribute")]
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::nm_tc_qdisc_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    /// Sets the queueing discipline handle.
    /// ## `handle`
    /// the queueing discipline handle
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_tc_qdisc_set_handle")]
    pub fn set_handle(&self, handle: u32) {
        unsafe {
            ffi::nm_tc_qdisc_set_handle(self.to_glib_none().0, handle);
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
impl PartialEq for TCQdisc {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TCQdisc {}
