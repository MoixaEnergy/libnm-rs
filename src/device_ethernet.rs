// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{Device, Object};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    ///
    ///
    /// ## Properties
    ///
    ///
    /// #### `carrier`
    ///  Whether the device has carrier.
    ///
    /// Readable
    ///
    ///
    /// #### `perm-hw-address`
    ///  The permanent hardware (MAC) address of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `s390-subchannels`
    ///  Identifies subchannels of this network device used for
    /// communication with z/VM or s390 host.
    ///
    /// Readable
    ///
    ///
    /// #### `speed`
    ///  The speed of the device.
    ///
    /// Readable
    /// <details><summary><h4>Device</h4></summary>
    ///
    ///
    /// #### `active-connection`
    ///  The [`ActiveConnection`][crate::ActiveConnection] object that "owns" this device during activation.
    ///
    /// Readable
    ///
    ///
    /// #### `autoconnect`
    ///  Whether the device can auto-activate a connection.
    ///
    /// The property setter is a synchronous D-Bus call. This is deprecated since 1.22.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `available-connections`
    ///  The available connections of the device
    ///
    /// Readable
    ///
    ///
    /// #### `capabilities`
    ///  The capabilities of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `device-type`
    ///  The numeric type of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `dhcp4-config`
    ///  The IPv4 [`DhcpConfig`][crate::DhcpConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `dhcp6-config`
    ///  The IPv6 [`DhcpConfig`][crate::DhcpConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `driver`
    ///  The driver of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `driver-version`
    ///  The version of the device driver.
    ///
    /// Readable
    ///
    ///
    /// #### `firmware-missing`
    ///  When [`true`] indicates the device is likely missing firmware required
    /// for its operation.
    ///
    /// Readable
    ///
    ///
    /// #### `firmware-version`
    ///  The firmware version of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `hw-address`
    ///  The hardware address of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `interface`
    ///  The interface of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `interface-flags`
    ///  The interface flags.
    ///
    /// Readable
    ///
    ///
    /// #### `ip-interface`
    ///  The IP interface of the device which should be used for all IP-related
    /// operations like addressing and routing.
    ///
    /// Readable
    ///
    ///
    /// #### `ip4-config`
    ///  The `NMIP4Config` of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip4-connectivity`
    ///  The IPv4 connectivity state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip6-config`
    ///  The IPv6 [`IPConfig`][crate::IPConfig] of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `ip6-connectivity`
    ///  The IPv6 connectivity state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `lldp-neighbors`
    ///  The LLDP neighbors.
    ///
    /// Readable
    ///
    ///
    /// #### `managed`
    ///  Whether the device is managed by NetworkManager.
    ///
    /// Readable
    ///
    ///
    /// #### `metered`
    ///  Whether the device is metered.
    ///
    /// Readable
    ///
    ///
    /// #### `mtu`
    ///  The MTU of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `nm-plugin-missing`
    ///  When [`true`] indicates that the NetworkManager plugin for the device
    /// is not installed.
    ///
    /// Readable
    ///
    ///
    /// #### `path`
    ///  The device path as exposed by the udev property ID_PATH.
    ///
    /// The string is backslash escaped (C escaping) for invalid
    /// characters. The escaping can be reverted with `g_strcompress()`,
    /// however the result may not be valid UTF-8.
    ///
    /// Readable
    ///
    ///
    /// #### `physical-port-id`
    ///  The physical port ID of the device. (See
    /// [`DeviceExt::physical_port_id()`][crate::prelude::DeviceExt::physical_port_id()].)
    ///
    /// Readable
    ///
    ///
    /// #### `ports`
    ///  The port devices of the controller device. For devices that cannot be
    /// controllers this is likely to be always empty.
    ///
    /// Readable
    ///
    ///
    /// #### `product`
    ///  The product string of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `real`
    ///  Whether the device is real or is a placeholder device that could
    /// be created automatically by NetworkManager if one of its
    /// [`available-connections`][struct@crate::Device#available-connections] was activated.
    ///
    /// Readable
    ///
    ///
    /// #### `state`
    ///  The state of the device.
    ///
    /// Readable
    ///
    ///
    /// #### `state-reason`
    ///  The reason for the device state.
    ///
    /// Readable
    ///
    ///
    /// #### `udi`
    ///  An operating-system specific device hardware identifier; this is not
    /// unique to a specific hardware device across reboots or hotplugs. It
    /// is an opaque string which for some device types (Bluetooth, Modem)
    /// contains an identifier provided by the underlying hardware service daemon
    /// such as Bluez or ModemManager, and clients can use this property to
    /// request more information about the device from those services.
    ///
    /// Readable
    ///
    ///
    /// #### `vendor`
    ///  The vendor string of the device.
    ///
    /// Readable
    /// </details>
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
    /// [`DeviceEthernetExt`][trait@crate::prelude::DeviceEthernetExt], [`DeviceExt`][trait@crate::prelude::DeviceExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMDeviceEthernet")]
    pub struct DeviceEthernet(Object<ffi::NMDeviceEthernet, ffi::NMDeviceEthernetClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_ethernet_get_type(),
    }
}

impl DeviceEthernet {
    pub const NONE: Option<&'static DeviceEthernet> = None;
}

/// Trait containing all [`struct@DeviceEthernet`] methods.
///
/// # Implementors
///
/// [`DeviceEthernet`][struct@crate::DeviceEthernet], [`DeviceVeth`][struct@crate::DeviceVeth]
pub trait DeviceEthernetExt: 'static {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has carrier
    #[doc(alias = "nm_device_ethernet_get_carrier")]
    #[doc(alias = "get_carrier")]
    fn is_carrier(&self) -> bool;

    /// Gets the active hardware (MAC) address of the [`DeviceEthernet`][crate::DeviceEthernet]
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`DeviceExt::hw_address()`][crate::prelude::DeviceExt::hw_address()] instead.
    ///
    /// # Returns
    ///
    /// the active hardware address. This is the internal string used by the
    /// device, and must not be modified.
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[allow(deprecated)]
    #[doc(alias = "nm_device_ethernet_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    fn hw_address(&self) -> Option<glib::GString>;

    /// Gets the permanent hardware (MAC) address of the [`DeviceEthernet`][crate::DeviceEthernet]
    ///
    /// # Returns
    ///
    /// the permanent hardware address. This is the internal string used by the
    /// device, and must not be modified.
    #[doc(alias = "nm_device_ethernet_get_permanent_hw_address")]
    #[doc(alias = "get_permanent_hw_address")]
    fn permanent_hw_address(&self) -> Option<glib::GString>;

    /// Return the list of s390 subchannels if the device supports them.
    ///
    /// # Returns
    ///
    /// array of strings, each specifying
    ///  one subchannel the s390 device uses to communicate to the host.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_device_ethernet_get_s390_subchannels")]
    #[doc(alias = "get_s390_subchannels")]
    fn s390_subchannels(&self) -> Vec<glib::GString>;

    /// Gets the speed of the [`DeviceEthernet`][crate::DeviceEthernet] in Mbit/s.
    ///
    /// # Returns
    ///
    /// the speed of the device in Mbit/s
    #[doc(alias = "nm_device_ethernet_get_speed")]
    #[doc(alias = "get_speed")]
    fn speed(&self) -> u32;

    /// The permanent hardware (MAC) address of the device.
    #[doc(alias = "perm-hw-address")]
    fn perm_hw_address(&self) -> Option<glib::GString>;

    #[doc(alias = "carrier")]
    fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "perm-hw-address")]
    fn connect_perm_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "s390-subchannels")]
    fn connect_s390_subchannels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "speed")]
    fn connect_speed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceEthernet>> DeviceEthernetExt for O {
    fn is_carrier(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_device_ethernet_get_carrier(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn hw_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_device_ethernet_get_hw_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn permanent_hw_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_device_ethernet_get_permanent_hw_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn s390_subchannels(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_ethernet_get_s390_subchannels(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn speed(&self) -> u32 {
        unsafe { ffi::nm_device_ethernet_get_speed(self.as_ref().to_glib_none().0) }
    }

    fn perm_hw_address(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "perm-hw-address")
    }

    fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<
            P: IsA<DeviceEthernet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMDeviceEthernet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceEthernet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_perm_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_perm_hw_address_trampoline<
            P: IsA<DeviceEthernet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMDeviceEthernet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceEthernet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::perm-hw-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_perm_hw_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn connect_s390_subchannels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_s390_subchannels_trampoline<
            P: IsA<DeviceEthernet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMDeviceEthernet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceEthernet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::s390-subchannels\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_s390_subchannels_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_speed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_speed_trampoline<
            P: IsA<DeviceEthernet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMDeviceEthernet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceEthernet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::speed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_speed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceEthernet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceEthernet")
    }
}
