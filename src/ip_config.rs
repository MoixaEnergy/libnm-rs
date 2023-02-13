// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::{IPAddress, IPRoute, Object};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    ///
    ///
    /// This is an Abstract Base Class, you cannot instantiate it.
    ///
    /// ## Properties
    ///
    ///
    /// #### `addresses`
    ///  A [`glib::PtrArray`][crate::glib::PtrArray] containing the addresses ([`IPAddress`][crate::IPAddress]) of the configuration.
    ///
    /// Readable
    ///
    ///
    /// #### `domains`
    ///  The array containing domain strings of the configuration.
    ///
    /// Readable
    ///
    ///
    /// #### `family`
    ///  The IP address family of the configuration; either
    /// `<literal>`AF_INET`</literal>` or `<literal>`AF_INET6`</literal>`.
    ///
    /// Readable
    ///
    ///
    /// #### `gateway`
    ///  The IP gateway address of the configuration as string.
    ///
    /// Readable
    ///
    ///
    /// #### `nameservers`
    ///  The array containing name server IP addresses of the configuration.
    ///
    /// Readable
    ///
    ///
    /// #### `searches`
    ///  The array containing DNS search strings of the configuration.
    ///
    /// Readable
    ///
    ///
    /// #### `wins-servers`
    ///  The array containing WINS server IP addresses of the configuration.
    /// (This will always be empty for IPv6 configurations.)
    ///
    /// Readable
    /// <details><summary><h4>Object</h4></summary>
    ///
    ///
    /// #### `client`
    ///  The NMClient instance as returned by `nm_object_get_client()`.
    ///
    /// When an NMObject gets removed from the NMClient cache,
    /// the NMObject:path property stays unchanged, but this client
    /// instance gets reset to [`None`]. You can use this property to
    /// track removal of the object from the cache.
    ///
    /// Readable
    ///
    ///
    /// #### `path`
    ///  The D-Bus object path.
    ///
    /// The D-Bus path of an object instance never changes, even if the object
    /// gets removed from the cache. To see whether the object is still in the
    /// cache, check NMObject:client.
    ///
    /// Readable
    /// </details>
    ///
    /// # Implements
    ///
    /// [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMIPConfig")]
    pub struct IPConfig(Object<ffi::NMIPConfig, ffi::NMIPConfigClass>) @extends Object;

    match fn {
        type_ => || ffi::nm_ip_config_get_type(),
    }
}

impl IPConfig {
    /// Gets the IP addresses (containing the address, prefix, and gateway).
    ///
    /// # Returns
    ///
    /// the [`glib::PtrArray`][crate::glib::PtrArray]
    /// containing [`IPAddress`][crate::IPAddress]<!-- -->es. This is the internal copy used by the
    /// configuration and must not be modified. The library never modifies the
    /// returned array and thus it is safe for callers to reference and keep using it.
    #[doc(alias = "nm_ip_config_get_addresses")]
    #[doc(alias = "get_addresses")]
    pub fn addresses(&self) -> Vec<IPAddress> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_addresses(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the domain names.
    ///
    /// # Returns
    ///
    /// the array of domains.
    /// (This is never [`None`], though it may be 0-length).
    #[doc(alias = "nm_ip_config_get_domains")]
    #[doc(alias = "get_domains")]
    pub fn domains(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_domains(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the IP address family
    ///
    /// # Returns
    ///
    /// the IP address family; either `<literal>`AF_INET`</literal>` or
    /// `<literal>`AF_INET6`</literal>`
    #[doc(alias = "nm_ip_config_get_family")]
    #[doc(alias = "get_family")]
    pub fn family(&self) -> i32 {
        unsafe { ffi::nm_ip_config_get_family(self.to_glib_none().0) }
    }

    /// Gets the IP gateway address.
    ///
    /// # Returns
    ///
    /// the IP address of the gateway.
    #[doc(alias = "nm_ip_config_get_gateway")]
    #[doc(alias = "get_gateway")]
    pub fn gateway(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_ip_config_get_gateway(self.to_glib_none().0)) }
    }

    /// Gets the domain name servers (DNS).
    ///
    /// # Returns
    ///
    /// the array of nameserver IP addresses
    #[doc(alias = "nm_ip_config_get_nameservers")]
    #[doc(alias = "get_nameservers")]
    pub fn nameservers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_nameservers(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the routes.
    ///
    /// # Returns
    ///
    /// the [`glib::PtrArray`][crate::glib::PtrArray] containing
    /// [`IPRoute`][crate::IPRoute]<!-- -->s. This is the internal copy used by the configuration, and must
    /// not be modified. The library never modifies the returned array and thus it is
    /// safe for callers to reference and keep using it.
    #[doc(alias = "nm_ip_config_get_routes")]
    #[doc(alias = "get_routes")]
    pub fn routes(&self) -> Vec<IPRoute> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_routes(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the DNS searches.
    ///
    /// # Returns
    ///
    /// the array of DNS search strings.
    /// (This is never [`None`], though it may be 0-length).
    #[doc(alias = "nm_ip_config_get_searches")]
    #[doc(alias = "get_searches")]
    pub fn searches(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_searches(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the Windows Internet Name Service servers (WINS).
    ///
    /// # Returns
    ///
    /// the arry of WINS server IP address strings.
    /// (This is never [`None`], though it may be 0-length.)
    #[doc(alias = "nm_ip_config_get_wins_servers")]
    #[doc(alias = "get_wins_servers")]
    pub fn wins_servers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_ip_config_get_wins_servers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "addresses")]
    pub fn connect_addresses_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_addresses_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::addresses\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_addresses_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "domains")]
    pub fn connect_domains_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_domains_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::domains\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_domains_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "family")]
    pub fn connect_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gateway")]
    pub fn connect_gateway_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gateway_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::gateway\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gateway_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nameservers")]
    pub fn connect_nameservers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nameservers_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::nameservers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nameservers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "searches")]
    pub fn connect_searches_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_searches_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::searches\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_searches_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wins-servers")]
    pub fn connect_wins_servers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wins_servers_trampoline<F: Fn(&IPConfig) + 'static>(
            this: *mut ffi::NMIPConfig,
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
                b"notify::wins-servers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wins_servers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for IPConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IPConfig")
    }
}
