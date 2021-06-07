// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DnsEntry(Boxed<ffi::NMDnsEntry>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::nm_dns_entry_get_type(), ptr as *mut _) as *mut ffi::NMDnsEntry,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::nm_dns_entry_get_type(), ptr as *mut _),
        type_ => || ffi::nm_dns_entry_get_type(),
    }
}

impl DnsEntry {
    /// Gets the list of DNS domains.
    ///
    /// # Returns
    ///
    /// the list of DNS domains
    #[doc(alias = "nm_dns_entry_get_domains")]
    #[doc(alias = "get_domains")]
    pub fn domains(&mut self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_dns_entry_get_domains(
                self.to_glib_none_mut().0,
            ))
        }
    }

    /// Gets the interface on which name servers are contacted.
    ///
    /// # Returns
    ///
    /// the interface name
    #[doc(alias = "nm_dns_entry_get_interface")]
    #[doc(alias = "get_interface")]
    pub fn interface(&mut self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_dns_entry_get_interface(self.to_glib_none_mut().0)) }
    }

    /// Gets the list of name servers for this entry.
    ///
    /// # Returns
    ///
    /// the list of name servers
    #[doc(alias = "nm_dns_entry_get_nameservers")]
    #[doc(alias = "get_nameservers")]
    pub fn nameservers(&mut self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_dns_entry_get_nameservers(
                self.to_glib_none_mut().0,
            ))
        }
    }

    /// Gets the priority of the entry
    ///
    /// # Returns
    ///
    /// the priority of the entry
    #[doc(alias = "nm_dns_entry_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&mut self) -> i32 {
        unsafe { ffi::nm_dns_entry_get_priority(self.to_glib_none_mut().0) }
    }

    /// Gets whether the entry refers to VPN name servers.
    ///
    /// # Returns
    ///
    /// [`true`] if the entry refers to VPN name servers
    #[doc(alias = "nm_dns_entry_get_vpn")]
    #[doc(alias = "get_vpn")]
    pub fn is_vpn(&mut self) -> bool {
        unsafe { from_glib(ffi::nm_dns_entry_get_vpn(self.to_glib_none_mut().0)) }
    }
}
