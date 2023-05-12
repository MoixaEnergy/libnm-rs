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
    /// #### `ageing`
    ///  The lifetime in seconds of FDB entries learnt by the kernel.
    ///
    /// Readable
    ///
    ///
    /// #### `carrier`
    ///  Whether the device has carrier.
    ///
    /// This property is not implemented yet, and the property is always FALSE.
    ///
    /// Readable
    ///
    ///
    /// #### `dst-port`
    ///  The UDP destination port used to communicate with the remote VXLAN tunnel
    /// endpoint.
    ///
    /// Readable
    ///
    ///
    /// #### `group`
    ///  The unicast destination IP address used in outgoing packets when the
    /// destination link layer address is not known in the VXLAN device
    /// forwarding database or the multicast IP address joined.
    ///
    /// Readable
    ///
    ///
    /// #### `id`
    ///  The device's VXLAN ID.
    ///
    /// Readable
    ///
    ///
    /// #### `l2miss`
    ///  Whether netlink LL ADDR miss notifications are generated.
    ///
    /// Readable
    ///
    ///
    /// #### `l3miss`
    ///  Whether netlink IP ADDR miss notifications are generated.
    ///
    /// Readable
    ///
    ///
    /// #### `learning`
    ///  Whether unknown source link layer addresses and IP addresses are entered
    /// into the VXLAN device forwarding database.
    ///
    /// Readable
    ///
    ///
    /// #### `limit`
    ///  The maximum number of entries that can be added to the forwarding table.
    ///
    /// Readable
    ///
    ///
    /// #### `local`
    ///  The source IP address to use in outgoing packets.
    ///
    /// Readable
    ///
    ///
    /// #### `parent`
    ///  The devices's parent device.
    ///
    /// Readable
    ///
    ///
    /// #### `proxy`
    ///  Whether ARP proxy is turned on.
    ///
    /// Readable
    ///
    ///
    /// #### `rsc`
    ///  Whether route short circuit is turned on.
    ///
    /// Readable
    ///
    ///
    /// #### `src-port-max`
    ///  The maximum UDP source port used to communicate with the remote VXLAN
    /// tunnel endpoint.
    ///
    /// Readable
    ///
    ///
    /// #### `src-port-min`
    ///  The minimum UDP source port used to communicate with the remote VXLAN
    /// tunnel endpoint.
    ///
    /// Readable
    ///
    ///
    /// #### `tos`
    ///  The TOS value to use in outgoing packets.
    ///
    /// Readable
    ///
    ///
    /// #### `ttl`
    ///  The time-to-live value to use in outgoing packets.
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
    /// [`DeviceExt`][trait@crate::prelude::DeviceExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMDeviceVxlan")]
    pub struct DeviceVxlan(Object<ffi::NMDeviceVxlan, ffi::NMDeviceVxlanClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_vxlan_get_type(),
    }
}

impl DeviceVxlan {
    ///
    /// # Returns
    ///
    /// the lifetime in seconds of FDB entries learnt by the kernel
    #[doc(alias = "nm_device_vxlan_get_ageing")]
    #[doc(alias = "get_ageing")]
    pub fn ageing(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_ageing(self.to_glib_none().0) }
    }

    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has carrier.
    ///
    /// This property is not implemented yet, and the function always returns
    /// FALSE.
    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "nm_device_vxlan_get_carrier")]
    #[doc(alias = "get_carrier")]
    pub fn is_carrier(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_carrier(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the UDP destination port
    #[doc(alias = "nm_device_vxlan_get_dst_port")]
    #[doc(alias = "get_dst_port")]
    pub fn dst_port(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_dst_port(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// The unicast destination IP address or the multicast
    /// IP address joined
    #[doc(alias = "nm_device_vxlan_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_group(self.to_glib_none().0)) }
    }

    /// Gets the hardware (MAC) address of the [`DeviceVxlan`][crate::DeviceVxlan]
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`DeviceExt::hw_address()`][crate::prelude::DeviceExt::hw_address()] instead.
    ///
    /// # Returns
    ///
    /// the hardware address. This is the internal string used by the
    /// device, and must not be modified.
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[allow(deprecated)]
    #[doc(alias = "nm_device_vxlan_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_hw_address(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's VXLAN ID.
    #[doc(alias = "nm_device_vxlan_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_id(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// whether netlink LL ADDR miss notifications are generated
    #[doc(alias = "nm_device_vxlan_get_l2miss")]
    #[doc(alias = "get_l2miss")]
    pub fn is_l2miss(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_l2miss(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether netlink IP ADDR miss notifications are generated
    #[doc(alias = "nm_device_vxlan_get_l3miss")]
    #[doc(alias = "get_l3miss")]
    pub fn is_l3miss(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_l3miss(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether address learning is enabled
    #[doc(alias = "nm_device_vxlan_get_learning")]
    #[doc(alias = "get_learning")]
    pub fn is_learning(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_learning(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the maximum number of entries that can be added to the
    /// forwarding table
    #[doc(alias = "nm_device_vxlan_get_limit")]
    #[doc(alias = "get_limit")]
    pub fn limit(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_limit(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the source IP address to use in outgoing packets
    #[doc(alias = "nm_device_vxlan_get_local")]
    #[doc(alias = "get_local")]
    pub fn local(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_local(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[doc(alias = "nm_device_vxlan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_vxlan_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether ARP proxy is turned on
    #[doc(alias = "nm_device_vxlan_get_proxy")]
    #[doc(alias = "get_proxy")]
    pub fn is_proxy(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_proxy(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// whether route short circuit is turned on
    #[cfg(any(feature = "v1_42", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_42")))]
    #[doc(alias = "nm_device_vxlan_get_rsc")]
    #[doc(alias = "get_rsc")]
    pub fn is_rsc(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vxlan_get_rsc(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the maximum UDP source port
    #[doc(alias = "nm_device_vxlan_get_src_port_max")]
    #[doc(alias = "get_src_port_max")]
    pub fn src_port_max(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_src_port_max(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the minimum UDP source port
    #[doc(alias = "nm_device_vxlan_get_src_port_min")]
    #[doc(alias = "get_src_port_min")]
    pub fn src_port_min(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_src_port_min(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the TOS value to use in outgoing packets
    #[doc(alias = "nm_device_vxlan_get_tos")]
    #[doc(alias = "get_tos")]
    pub fn tos(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_tos(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the time-to-live value to use in outgoing packets
    #[doc(alias = "nm_device_vxlan_get_ttl")]
    #[doc(alias = "get_ttl")]
    pub fn ttl(&self) -> u32 {
        unsafe { ffi::nm_device_vxlan_get_ttl(self.to_glib_none().0) }
    }

    /// Whether the device has carrier.
    ///
    /// This property is not implemented yet, and the property is always FALSE.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn get_property_carrier(&self) -> bool {
        glib::ObjectExt::property(self, "carrier")
    }

    /// Whether route short circuit is turned on.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn get_property_rsc(&self) -> bool {
        glib::ObjectExt::property(self, "rsc")
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "ageing")]
    pub fn connect_ageing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ageing_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::ageing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ageing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "carrier")]
    pub fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "dst-port")]
    pub fn connect_dst_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dst_port_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::dst-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dst_port_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "group")]
    pub fn connect_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "id")]
    pub fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "l2miss")]
    pub fn connect_l2miss_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_l2miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::l2miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l2miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "l3miss")]
    pub fn connect_l3miss_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_l3miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::l3miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l3miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "learning")]
    pub fn connect_learning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_learning_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::learning\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_learning_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "limit")]
    pub fn connect_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_limit_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_limit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "local")]
    pub fn connect_local_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "proxy")]
    pub fn connect_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "rsc")]
    pub fn connect_rsc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rsc_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::rsc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rsc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "src-port-max")]
    pub fn connect_src_port_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_max_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::src-port-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_max_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "src-port-min")]
    pub fn connect_src_port_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_min_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::src-port-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_min_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "tos")]
    pub fn connect_tos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tos_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::tos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "ttl")]
    pub fn connect_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut ffi::NMDeviceVxlan,
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
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceVxlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceVxlan")
    }
}
