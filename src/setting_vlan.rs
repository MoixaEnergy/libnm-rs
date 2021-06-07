// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::VlanFlags;
use crate::VlanPriorityMap;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingVlan")]
    pub struct SettingVlan(Object<ffi::NMSettingVlan, ffi::NMSettingVlanClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_vlan_get_type(),
    }
}

impl SettingVlan {
    /// Creates a new [`SettingVlan`][crate::SettingVlan] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingVlan`][crate::SettingVlan] object
    #[doc(alias = "nm_setting_vlan_new")]
    pub fn new() -> SettingVlan {
        unsafe { Setting::from_glib_full(ffi::nm_setting_vlan_new()).unsafe_cast() }
    }

    /// Adds a priority mapping to the `property::SettingVlan::ingress_priority_map` or
    /// `property::SettingVlan::egress_priority_map` properties of the setting. If `from` is
    /// already in the given priority map, this function will overwrite the
    /// existing entry with the new `to`.
    ///
    /// If `map` is [`VlanPriorityMap::IngressMap`][crate::VlanPriorityMap::IngressMap] then `from` is the incoming 802.1q VLAN
    /// Priority Code Point (PCP) value, and `to` is the Linux SKB priority value.
    ///
    /// If `map` is [`VlanPriorityMap::EgressMap`][crate::VlanPriorityMap::EgressMap] then `from` is the Linux SKB priority value and
    /// `to` is the outgoing 802.1q VLAN Priority Code Point (PCP) value.
    /// ## `map`
    /// the type of priority map
    /// ## `from`
    /// the priority to map to `to`
    /// ## `to`
    /// the priority to map `from` to
    ///
    /// # Returns
    ///
    /// [`true`].
    #[doc(alias = "nm_setting_vlan_add_priority")]
    pub fn add_priority(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_vlan_add_priority(
                self.to_glib_none().0,
                map.into_glib(),
                from,
                to,
            ))
        }
    }

    /// Adds a priority map entry into either the `property::SettingVlan::ingress_priority_map`
    /// or the `property::SettingVlan::egress_priority_map` properties. The priority map maps
    /// the Linux SKB priorities to 802.1p priorities.
    /// ## `map`
    /// the type of priority map
    /// ## `str`
    /// the string which contains a priority map, like "3:7"
    ///
    /// # Returns
    ///
    /// [`true`] if the entry was successfully added to the list, or it
    /// overwrote the old value, [`false`] if `str` is not a valid mapping.
    #[doc(alias = "nm_setting_vlan_add_priority_str")]
    pub fn add_priority_str(&self, map: VlanPriorityMap, str: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_vlan_add_priority_str(
                self.to_glib_none().0,
                map.into_glib(),
                str.to_glib_none().0,
            ))
        }
    }

    /// Clear all the entries from `property::SettingVlan::ingress_priority_map` or
    /// `property::SettingVlan::egress_priority_map` properties.
    /// ## `map`
    /// the type of priority map
    #[doc(alias = "nm_setting_vlan_clear_priorities")]
    pub fn clear_priorities(&self, map: VlanPriorityMap) {
        unsafe {
            ffi::nm_setting_vlan_clear_priorities(self.to_glib_none().0, map.into_glib());
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingVlan::flags` property of the setting
    #[doc(alias = "nm_setting_vlan_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> u32 {
        unsafe { ffi::nm_setting_vlan_get_flags(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingVlan::id` property of the setting
    #[doc(alias = "nm_setting_vlan_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u32 {
        unsafe { ffi::nm_setting_vlan_get_id(self.to_glib_none().0) }
    }

    /// Returns the number of entries in the
    /// `property::SettingVlan::ingress_priority_map` or `property::SettingVlan::egress_priority_map`
    /// properties of this setting.
    /// ## `map`
    /// the type of priority map
    ///
    /// # Returns
    ///
    /// return the number of ingress/egress priority entries.
    #[doc(alias = "nm_setting_vlan_get_num_priorities")]
    #[doc(alias = "get_num_priorities")]
    pub fn num_priorities(&self, map: VlanPriorityMap) -> i32 {
        unsafe { ffi::nm_setting_vlan_get_num_priorities(self.to_glib_none().0, map.into_glib()) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingVlan::parent` property of the setting
    #[doc(alias = "nm_setting_vlan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_vlan_get_parent(self.to_glib_none().0)) }
    }

    /// Retrieve one of the entries of the `property::SettingVlan::ingress_priority_map`
    /// or `property::SettingVlan::egress_priority_map` properties of this setting.
    /// ## `map`
    /// the type of priority map
    /// ## `idx`
    /// the zero-based index of the ingress/egress priority map entry
    ///
    /// # Returns
    ///
    /// returns [`true`] if `idx` is in range. Otherwise, [`false`].
    ///
    /// ## `out_from`
    /// on return the value of the priority map's 'from' item
    ///
    /// ## `out_to`
    /// on return the value of priority map's 'to' item
    #[doc(alias = "nm_setting_vlan_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&self, map: VlanPriorityMap, idx: u32) -> Option<(u32, u32)> {
        unsafe {
            let mut out_from = mem::MaybeUninit::uninit();
            let mut out_to = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::nm_setting_vlan_get_priority(
                self.to_glib_none().0,
                map.into_glib(),
                idx,
                out_from.as_mut_ptr(),
                out_to.as_mut_ptr(),
            ));
            let out_from = out_from.assume_init();
            let out_to = out_to.assume_init();
            if ret {
                Some((out_from, out_to))
            } else {
                None
            }
        }
    }

    /// Removes the priority map at index `idx` from the
    /// `property::SettingVlan::ingress_priority_map` or `property::SettingVlan::egress_priority_map`
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `idx`
    /// the zero-based index of the priority map to remove
    #[doc(alias = "nm_setting_vlan_remove_priority")]
    pub fn remove_priority(&self, map: VlanPriorityMap, idx: u32) {
        unsafe {
            ffi::nm_setting_vlan_remove_priority(self.to_glib_none().0, map.into_glib(), idx);
        }
    }

    /// Removes the priority map `form`:`to` from the `property::SettingVlan::ingress_priority_map`
    /// or `property::SettingVlan::egress_priority_map` (according to `map` argument)
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `from`
    /// the priority to map to `to`
    /// ## `to`
    /// the priority to map `from` to
    ///
    /// # Returns
    ///
    /// [`true`] if the priority mapping was found and removed; [`false`] if it was not.
    #[doc(alias = "nm_setting_vlan_remove_priority_by_value")]
    pub fn remove_priority_by_value(&self, map: VlanPriorityMap, from: u32, to: u32) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_vlan_remove_priority_by_value(
                self.to_glib_none().0,
                map.into_glib(),
                from,
                to,
            ))
        }
    }

    /// Removes the priority map `str` from the `property::SettingVlan::ingress_priority_map`
    /// or `property::SettingVlan::egress_priority_map` (according to `map` argument)
    /// properties.
    /// ## `map`
    /// the type of priority map
    /// ## `str`
    /// the string which contains a priority map, like "3:7"
    ///
    /// # Returns
    ///
    /// [`true`] if the priority mapping was found and removed; [`false`] if it was not.
    #[doc(alias = "nm_setting_vlan_remove_priority_str_by_value")]
    pub fn remove_priority_str_by_value(&self, map: VlanPriorityMap, str: &str) -> bool {
        unsafe {
            from_glib(ffi::nm_setting_vlan_remove_priority_str_by_value(
                self.to_glib_none().0,
                map.into_glib(),
                str.to_glib_none().0,
            ))
        }
    }

    /// For outgoing packets, a list of mappings from Linux SKB priorities to
    /// 802.1p priorities. The mapping is given in the format "from:to" where
    /// both "from" and "to" are unsigned integers, ie "7:3".
    #[doc(alias = "egress-priority-map")]
    pub fn egress_priority_map(&self) -> Vec<glib::GString> {
        unsafe {
            let mut value =
                glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"egress-priority-map\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `egress-priority-map` getter")
        }
    }

    /// For outgoing packets, a list of mappings from Linux SKB priorities to
    /// 802.1p priorities. The mapping is given in the format "from:to" where
    /// both "from" and "to" are unsigned integers, ie "7:3".
    #[doc(alias = "egress-priority-map")]
    pub fn set_egress_priority_map(&self, egress_priority_map: &[&str]) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"egress-priority-map\0".as_ptr() as *const _,
                egress_priority_map.to_value().to_glib_none().0,
            );
        }
    }

    /// One or more flags which control the behavior and features of the VLAN
    /// interface. Flags include [`VlanFlags::REORDER_HEADERS`][crate::VlanFlags::REORDER_HEADERS] (reordering of
    /// output packet headers), [`VlanFlags::GVRP`][crate::VlanFlags::GVRP] (use of the GVRP protocol),
    /// and [`VlanFlags::LOOSE_BINDING`][crate::VlanFlags::LOOSE_BINDING] (loose binding of the interface to its
    /// master device's operating state). [`VlanFlags::MVRP`][crate::VlanFlags::MVRP] (use of the MVRP
    /// protocol).
    ///
    /// The default value of this property is NM_VLAN_FLAG_REORDER_HEADERS,
    /// but it used to be 0. To preserve backward compatibility, the default-value
    /// in the D-Bus API continues to be 0 and a missing property on D-Bus
    /// is still considered as 0.
    pub fn set_flags(&self, flags: VlanFlags) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"flags\0".as_ptr() as *const _,
                flags.to_value().to_glib_none().0,
            );
        }
    }

    /// The VLAN identifier that the interface created by this connection should
    /// be assigned. The valid range is from 0 to 4094, without the reserved id 4095.
    pub fn set_id(&self, id: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"id\0".as_ptr() as *const _,
                id.to_value().to_glib_none().0,
            );
        }
    }

    /// For incoming packets, a list of mappings from 802.1p priorities to Linux
    /// SKB priorities. The mapping is given in the format "from:to" where both
    /// "from" and "to" are unsigned integers, ie "7:3".
    #[doc(alias = "ingress-priority-map")]
    pub fn ingress_priority_map(&self) -> Vec<glib::GString> {
        unsafe {
            let mut value =
                glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ingress-priority-map\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ingress-priority-map` getter")
        }
    }

    /// For incoming packets, a list of mappings from 802.1p priorities to Linux
    /// SKB priorities. The mapping is given in the format "from:to" where both
    /// "from" and "to" are unsigned integers, ie "7:3".
    #[doc(alias = "ingress-priority-map")]
    pub fn set_ingress_priority_map(&self, ingress_priority_map: &[&str]) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"ingress-priority-map\0".as_ptr() as *const _,
                ingress_priority_map.to_value().to_glib_none().0,
            );
        }
    }

    /// If given, specifies the parent interface name or parent connection UUID
    /// from which this VLAN interface should be created. If this property is
    /// not specified, the connection must contain an [`SettingWired`][crate::SettingWired] setting
    /// with a `property::SettingWired::mac-address` property.
    pub fn set_parent(&self, parent: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"parent\0".as_ptr() as *const _,
                parent.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "egress-priority-map")]
    pub fn connect_egress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_egress_priority_map_trampoline<
            F: Fn(&SettingVlan) + 'static,
        >(
            this: *mut ffi::NMSettingVlan,
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
                b"notify::egress-priority-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_egress_priority_map_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&SettingVlan) + 'static>(
            this: *mut ffi::NMSettingVlan,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "id")]
    pub fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&SettingVlan) + 'static>(
            this: *mut ffi::NMSettingVlan,
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

    #[doc(alias = "ingress-priority-map")]
    pub fn connect_ingress_priority_map_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ingress_priority_map_trampoline<
            F: Fn(&SettingVlan) + 'static,
        >(
            this: *mut ffi::NMSettingVlan,
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
                b"notify::ingress-priority-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ingress_priority_map_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&SettingVlan) + 'static>(
            this: *mut ffi::NMSettingVlan,
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
}

impl Default for SettingVlan {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingVlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingVlan")
    }
}
