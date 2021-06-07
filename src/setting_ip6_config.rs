// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use crate::SettingIP6ConfigAddrGenMode;
use crate::SettingIP6ConfigPrivacy;
use crate::SettingIPConfig;
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
    #[doc(alias = "NMSettingIP6Config")]
    pub struct SettingIP6Config(Object<ffi::NMSettingIP6Config, ffi::NMSettingIP6ConfigClass>) @extends SettingIPConfig, Setting;

    match fn {
        type_ => || ffi::nm_setting_ip6_config_get_type(),
    }
}

impl SettingIP6Config {
    /// Creates a new [`SettingIP6Config`][crate::SettingIP6Config] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingIP6Config`][crate::SettingIP6Config] object
    #[doc(alias = "nm_setting_ip6_config_new")]
    pub fn new() -> SettingIP6Config {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ip6_config_new()).unsafe_cast() }
    }

    /// Returns the value contained in the `property::SettingIP6Config::addr-gen-mode`
    /// property.
    ///
    /// # Returns
    ///
    /// IPv6 Address Generation Mode.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_setting_ip6_config_get_addr_gen_mode")]
    #[doc(alias = "get_addr_gen_mode")]
    pub fn addr_gen_mode(&self) -> SettingIP6ConfigAddrGenMode {
        unsafe {
            from_glib(ffi::nm_setting_ip6_config_get_addr_gen_mode(
                self.to_glib_none().0,
            ))
        }
    }

    /// Returns the value contained in the `property::SettingIP6Config::dhcp-duid`
    /// property.
    ///
    /// # Returns
    ///
    /// The configured DUID value to be included in the DHCPv6 requests
    /// sent to the DHCPv6 servers.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_setting_ip6_config_get_dhcp_duid")]
    #[doc(alias = "get_dhcp_duid")]
    pub fn dhcp_duid(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_ip6_config_get_dhcp_duid(
                self.to_glib_none().0,
            ))
        }
    }

    /// Returns the value contained in the `property::SettingIP6Config::ip6-privacy`
    /// property.
    ///
    /// # Returns
    ///
    /// IPv6 Privacy Extensions configuration value ([`SettingIP6ConfigPrivacy`][crate::SettingIP6ConfigPrivacy]).
    #[doc(alias = "nm_setting_ip6_config_get_ip6_privacy")]
    #[doc(alias = "get_ip6_privacy")]
    pub fn ip6_privacy(&self) -> SettingIP6ConfigPrivacy {
        unsafe {
            from_glib(ffi::nm_setting_ip6_config_get_ip6_privacy(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// The configured [`SETTING_IP6_CONFIG_RA_TIMEOUT`][crate::SETTING_IP6_CONFIG_RA_TIMEOUT] value with the
    /// timeout for router advertisements in seconds.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_setting_ip6_config_get_ra_timeout")]
    #[doc(alias = "get_ra_timeout")]
    pub fn ra_timeout(&self) -> i32 {
        unsafe { ffi::nm_setting_ip6_config_get_ra_timeout(self.to_glib_none().0) }
    }

    /// Returns the value contained in the `property::SettingIP6Config::token`
    /// property.
    ///
    /// # Returns
    ///
    /// A string.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "nm_setting_ip6_config_get_token")]
    #[doc(alias = "get_token")]
    pub fn token(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_ip6_config_get_token(self.to_glib_none().0)) }
    }

    /// Configure method for creating the address for use with RFC4862 IPv6
    /// Stateless Address Autoconfiguration. The permitted values are:
    /// [`SettingIP6ConfigAddrGenMode::Eui64`][crate::SettingIP6ConfigAddrGenMode::Eui64] or
    /// [`SettingIP6ConfigAddrGenMode::StablePrivacy`][crate::SettingIP6ConfigAddrGenMode::StablePrivacy].
    ///
    /// If the property is set to EUI64, the addresses will be generated
    /// using the interface tokens derived from hardware address. This makes
    /// the host part of the address to stay constant, making it possible
    /// to track host's presence when it changes networks. The address changes
    /// when the interface hardware is replaced.
    ///
    /// The value of stable-privacy enables use of cryptographically
    /// secure hash of a secret host-specific key along with the connection's
    /// stable-id and the network address as specified by RFC7217.
    /// This makes it impossible to use the address track host's presence,
    /// and makes the address stable when the network interface hardware is
    /// replaced.
    ///
    /// On D-Bus, the absence of an addr-gen-mode setting equals enabling
    /// stable-privacy. For keyfile plugin, the absence of the setting
    /// on disk means EUI64 so that the property doesn't change on upgrade
    /// from older versions.
    ///
    /// Note that this setting is distinct from the Privacy Extensions as
    /// configured by "ip6-privacy" property and it does not affect the
    /// temporary addresses configured with this option.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "addr-gen-mode")]
    pub fn set_addr_gen_mode(&self, addr_gen_mode: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"addr-gen-mode\0".as_ptr() as *const _,
                addr_gen_mode.to_value().to_glib_none().0,
            );
        }
    }

    /// A string containing the DHCPv6 Unique Identifier (DUID) used by the dhcp
    /// client to identify itself to DHCPv6 servers (RFC 3315). The DUID is carried
    /// in the Client Identifier option.
    /// If the property is a hex string ('aa:bb:cc') it is interpreted as a binary
    /// DUID and filled as an opaque value in the Client Identifier option.
    ///
    /// The special value "lease" will retrieve the DUID previously used from the
    /// lease file belonging to the connection. If no DUID is found and "dhclient"
    /// is the configured dhcp client, the DUID is searched in the system-wide
    /// dhclient lease file. If still no DUID is found, or another dhcp client is
    /// used, a global and permanent DUID-UUID (RFC 6355) will be generated based
    /// on the machine-id.
    ///
    /// The special values "llt" and "ll" will generate a DUID of type LLT or LL
    /// (see RFC 3315) based on the current MAC address of the device. In order to
    /// try providing a stable DUID-LLT, the time field will contain a constant
    /// timestamp that is used globally (for all profiles) and persisted to disk.
    ///
    /// The special values "stable-llt", "stable-ll" and "stable-uuid" will generate
    /// a DUID of the corresponding type, derived from the connection's stable-id and
    /// a per-host unique key. You may want to include the "${DEVICE}" or "${MAC}" specifier
    /// in the stable-id, in case this profile gets activated on multiple devices.
    /// So, the link-layer address of "stable-ll" and "stable-llt" will be a generated
    /// address derived from the stable id. The DUID-LLT time value in the "stable-llt"
    /// option will be picked among a static timespan of three years (the upper bound
    /// of the interval is the same constant timestamp used in "llt").
    ///
    /// When the property is unset, the global value provided for "ipv6.dhcp-duid" is
    /// used. If no global value is provided, the default "lease" value is assumed.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "dhcp-duid")]
    pub fn set_dhcp_duid(&self, dhcp_duid: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"dhcp-duid\0".as_ptr() as *const _,
                dhcp_duid.to_value().to_glib_none().0,
            );
        }
    }

    /// Configure IPv6 Privacy Extensions for SLAAC, described in RFC4941. If
    /// enabled, it makes the kernel generate a temporary IPv6 address in
    /// addition to the public one generated from MAC address via modified
    /// EUI-64. This enhances privacy, but could cause problems in some
    /// applications, on the other hand. The permitted values are: -1: unknown,
    /// 0: disabled, 1: enabled (prefer public address), 2: enabled (prefer temporary
    /// addresses).
    ///
    /// Having a per-connection setting set to "-1" (unknown) means fallback to
    /// global configuration "ipv6.ip6-privacy".
    ///
    /// If also global configuration is unspecified or set to "-1", fallback to read
    /// "/proc/sys/net/ipv6/conf/default/use_tempaddr".
    ///
    /// Note that this setting is distinct from the Stable Privacy addresses
    /// that can be enabled with the "addr-gen-mode" property's "stable-privacy"
    /// setting as another way of avoiding host tracking with IPv6 addresses.
    #[doc(alias = "ip6-privacy")]
    pub fn set_ip6_privacy(&self, ip6_privacy: SettingIP6ConfigPrivacy) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ip6-privacy\0".as_ptr() as *const _,
                ip6_privacy.to_value().to_glib_none().0,
            );
        }
    }

    /// A timeout for waiting Router Advertisements in seconds. If zero (the default), a
    /// globally configured default is used. If still unspecified, the timeout depends on the
    /// sysctl settings of the device.
    ///
    /// Set to 2147483647 (MAXINT32) for infinity.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ra-timeout")]
    pub fn set_ra_timeout(&self, ra_timeout: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ra-timeout\0".as_ptr() as *const _,
                ra_timeout.to_value().to_glib_none().0,
            );
        }
    }

    /// Configure the token for draft-chown-6man-tokenised-ipv6-identifiers-02
    /// IPv6 tokenized interface identifiers. Useful with eui64 addr-gen-mode.
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    pub fn set_token(&self, token: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"token\0".as_ptr() as *const _,
                token.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "addr-gen-mode")]
    pub fn connect_addr_gen_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_addr_gen_mode_trampoline<F: Fn(&SettingIP6Config) + 'static>(
            this: *mut ffi::NMSettingIP6Config,
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
                b"notify::addr-gen-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_addr_gen_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "dhcp-duid")]
    pub fn connect_dhcp_duid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp_duid_trampoline<F: Fn(&SettingIP6Config) + 'static>(
            this: *mut ffi::NMSettingIP6Config,
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
                b"notify::dhcp-duid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dhcp_duid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ip6-privacy")]
    pub fn connect_ip6_privacy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_privacy_trampoline<F: Fn(&SettingIP6Config) + 'static>(
            this: *mut ffi::NMSettingIP6Config,
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
                b"notify::ip6-privacy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip6_privacy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ra-timeout")]
    pub fn connect_ra_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ra_timeout_trampoline<F: Fn(&SettingIP6Config) + 'static>(
            this: *mut ffi::NMSettingIP6Config,
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
                b"notify::ra-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ra_timeout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "token")]
    pub fn connect_token_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_token_trampoline<F: Fn(&SettingIP6Config) + 'static>(
            this: *mut ffi::NMSettingIP6Config,
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
                b"notify::token\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_token_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingIP6Config {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingIP6Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingIP6Config")
    }
}
