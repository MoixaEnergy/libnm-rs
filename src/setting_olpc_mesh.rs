// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Setting;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SettingOlpcMesh(Object<nm_sys::NMSettingOlpcMesh, nm_sys::NMSettingOlpcMeshClass, SettingOlpcMeshClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_olpc_mesh_get_type(),
    }
}

impl SettingOlpcMesh {
    /// Creates a new `SettingOlpcMesh` object with default values.
    ///
    /// # Returns
    ///
    /// the new empty `SettingOlpcMesh` object
    pub fn new() -> SettingOlpcMesh {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_olpc_mesh_new()).unsafe_cast() }
    }
}

impl Default for SettingOlpcMesh {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_OLPC_MESH: Option<&SettingOlpcMesh> = None;

/// Trait containing all `SettingOlpcMesh` methods.
///
/// # Implementors
///
/// [`SettingOlpcMesh`](struct.SettingOlpcMesh.html)
pub trait SettingOlpcMeshExt: 'static {
    fn get_channel(&self) -> u32;

    fn get_dhcp_anycast_address(&self) -> Option<GString>;

    fn get_ssid(&self) -> Option<glib::Bytes>;

    /// Channel on which the mesh network to join is located.
    fn set_property_channel(&self, channel: u32);

    /// Anycast DHCP MAC address used when requesting an IP address via DHCP.
    /// The specific anycast address used determines which DHCP server class
    /// answers the request.
    fn set_property_dhcp_anycast_address(&self, dhcp_anycast_address: Option<&str>);

    /// SSID of the mesh network to join.
    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>);

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dhcp_anycast_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingOlpcMesh>> SettingOlpcMeshExt for O {
    fn get_channel(&self) -> u32 {
        unsafe { nm_sys::nm_setting_olpc_mesh_get_channel(self.as_ref().to_glib_none().0) }
    }

    fn get_dhcp_anycast_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_olpc_mesh_get_dhcp_anycast_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ssid(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_olpc_mesh_get_ssid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_channel(&self, channel: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"channel\0".as_ptr() as *const _,
                Value::from(&channel).to_glib_none().0,
            );
        }
    }

    fn set_property_dhcp_anycast_address(&self, dhcp_anycast_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"dhcp-anycast-address\0".as_ptr() as *const _,
                Value::from(dhcp_anycast_address).to_glib_none().0,
            );
        }
    }

    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ssid\0".as_ptr() as *const _,
                Value::from(ssid).to_glib_none().0,
            );
        }
    }

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_channel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingOlpcMesh,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingOlpcMesh>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingOlpcMesh::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::channel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_channel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_dhcp_anycast_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp_anycast_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingOlpcMesh,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingOlpcMesh>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingOlpcMesh::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp-anycast-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dhcp_anycast_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ssid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingOlpcMesh,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingOlpcMesh>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingOlpcMesh::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ssid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ssid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingOlpcMesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingOlpcMesh")
    }
}
