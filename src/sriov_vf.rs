// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use crate::SriovVFVlanProtocol;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use std::mem;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct SriovVF(Shared<ffi::NMSriovVF>);

    match fn {
        ref => |ptr| ffi::nm_sriov_vf_ref(ptr),
        unref => |ptr| ffi::nm_sriov_vf_unref(ptr),
        type_ => || ffi::nm_sriov_vf_get_type(),
    }
}

impl SriovVF {
    /// Creates a new [`SriovVF`][crate::SriovVF] object.
    /// ## `index`
    /// the VF index
    ///
    /// # Returns
    ///
    /// the new [`SriovVF`][crate::SriovVF] object.
    #[doc(alias = "nm_sriov_vf_new")]
    pub fn new(index: u32) -> SriovVF {
        unsafe { from_glib_full(ffi::nm_sriov_vf_new(index)) }
    }

    /// Adds a VLAN to the VF.
    /// ## `vlan_id`
    /// the VLAN id
    ///
    /// # Returns
    ///
    /// [`true`] if the VLAN was added; [`false`] if it already existed
    #[doc(alias = "nm_sriov_vf_add_vlan")]
    pub fn add_vlan(&self, vlan_id: u32) -> bool {
        unsafe { from_glib(ffi::nm_sriov_vf_add_vlan(self.to_glib_none().0, vlan_id)) }
    }

    /// Creates a copy of `self`.
    ///
    /// # Returns
    ///
    /// a copy of `self`
    #[doc(alias = "nm_sriov_vf_dup")]
    pub fn dup(&self) -> Option<SriovVF> {
        unsafe { from_glib_full(ffi::nm_sriov_vf_dup(self.to_glib_none().0)) }
    }

    /// Determines if two [`SriovVF`][crate::SriovVF] objects have the same index,
    /// attributes and VLANs.
    /// ## `other`
    /// the [`SriovVF`][crate::SriovVF] to compare `self` to.
    ///
    /// # Returns
    ///
    /// [`true`] if the objects contain the same values, [`false`]
    ///  if they do not.
    #[doc(alias = "nm_sriov_vf_equal")]
    fn equal(&self, other: &SriovVF) -> bool {
        unsafe {
            from_glib(ffi::nm_sriov_vf_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    /// Gets the value of the attribute with name `name` on `self`
    /// ## `name`
    /// the name of a VF attribute
    ///
    /// # Returns
    ///
    /// the value of the attribute with name `name` on
    ///  `self`, or [`None`] if `self` has no such attribute.
    #[doc(alias = "nm_sriov_vf_get_attribute")]
    #[doc(alias = "get_attribute")]
    pub fn attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_sriov_vf_get_attribute(
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
    #[doc(alias = "nm_sriov_vf_get_attribute_names")]
    #[doc(alias = "get_attribute_names")]
    pub fn attribute_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::nm_sriov_vf_get_attribute_names(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the index property of this VF object.
    ///
    /// # Returns
    ///
    /// the VF index
    #[doc(alias = "nm_sriov_vf_get_index")]
    #[doc(alias = "get_index")]
    pub fn index(&self) -> u32 {
        unsafe { ffi::nm_sriov_vf_get_index(self.to_glib_none().0) }
    }

    /// Returns the VLANs currently configured on the VF.
    ///
    /// # Returns
    ///
    /// a list of VLAN ids configured on the VF.
    #[doc(alias = "nm_sriov_vf_get_vlan_ids")]
    #[doc(alias = "get_vlan_ids")]
    pub fn vlan_ids(&self) -> Vec<u32> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::nm_sriov_vf_get_vlan_ids(self.to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "nm_sriov_vf_get_vlan_protocol")]
    #[doc(alias = "get_vlan_protocol")]
    pub fn vlan_protocol(&self, vlan_id: u32) -> SriovVFVlanProtocol {
        unsafe {
            from_glib(ffi::nm_sriov_vf_get_vlan_protocol(
                self.to_glib_none().0,
                vlan_id,
            ))
        }
    }

    /// Returns the QoS value for the given VLAN.
    /// ## `vlan_id`
    /// the VLAN id
    ///
    /// # Returns
    ///
    /// the QoS value
    #[doc(alias = "nm_sriov_vf_get_vlan_qos")]
    #[doc(alias = "get_vlan_qos")]
    pub fn vlan_qos(&self, vlan_id: u32) -> u32 {
        unsafe { ffi::nm_sriov_vf_get_vlan_qos(self.to_glib_none().0, vlan_id) }
    }

    /// Removes a VLAN from a VF.
    /// ## `vlan_id`
    /// the VLAN id
    ///
    /// # Returns
    ///
    /// [`true`] if the VLAN was removed, [`false`] if the VLAN `vlan_id`
    ///  did not belong to the VF.
    #[doc(alias = "nm_sriov_vf_remove_vlan")]
    pub fn remove_vlan(&self, vlan_id: u32) -> bool {
        unsafe { from_glib(ffi::nm_sriov_vf_remove_vlan(self.to_glib_none().0, vlan_id)) }
    }

    /// Sets the named attribute on `self` to the given value.
    /// ## `name`
    /// the name of a route attribute
    /// ## `value`
    /// the value
    #[doc(alias = "nm_sriov_vf_set_attribute")]
    pub fn set_attribute(&self, name: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::nm_sriov_vf_set_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    /// Sets the protocol for the given VLAN.
    /// ## `vlan_id`
    /// the VLAN id
    /// ## `protocol`
    /// the VLAN protocol
    #[doc(alias = "nm_sriov_vf_set_vlan_protocol")]
    pub fn set_vlan_protocol(&self, vlan_id: u32, protocol: SriovVFVlanProtocol) {
        unsafe {
            ffi::nm_sriov_vf_set_vlan_protocol(
                self.to_glib_none().0,
                vlan_id,
                protocol.into_glib(),
            );
        }
    }

    /// Sets a QoS value for the given VLAN.
    /// ## `vlan_id`
    /// the VLAN id
    /// ## `qos`
    /// a QoS (priority) value
    #[doc(alias = "nm_sriov_vf_set_vlan_qos")]
    pub fn set_vlan_qos(&self, vlan_id: u32, qos: u32) {
        unsafe {
            ffi::nm_sriov_vf_set_vlan_qos(self.to_glib_none().0, vlan_id, qos);
        }
    }

    /// Validates a VF attribute, i.e. checks that the attribute is a known one,
    /// the value is of the correct type and well-formed.
    /// ## `name`
    /// the attribute name
    /// ## `value`
    /// the attribute value
    ///
    /// # Returns
    ///
    /// [`true`] if the attribute is valid, [`false`] otherwise
    ///
    /// ## `known`
    /// on return, whether the attribute name is a known one
    #[doc(alias = "nm_sriov_vf_attribute_validate")]
    pub fn attribute_validate(name: &str, value: &glib::Variant) -> Result<bool, glib::Error> {
        unsafe {
            let mut known = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::nm_sriov_vf_attribute_validate(
                name.to_glib_none().0,
                value.to_glib_none().0,
                known.as_mut_ptr(),
                &mut error,
            );
            let known = known.assume_init();
            if error.is_null() {
                Ok(from_glib(known))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl PartialEq for SriovVF {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for SriovVF {}
