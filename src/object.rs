// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
use crate::Client;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMObject")]
    pub struct Object(Object<ffi::NMObject, ffi::NMObjectClass>);

    match fn {
        type_ => || ffi::nm_object_get_type(),
    }
}

pub const NONE_OBJECT: Option<&Object> = None;

/// Trait containing all [`struct@Object`] methods.
///
/// # Implementors
///
/// [`AccessPoint`][struct@crate::AccessPoint], [`ActiveConnection`][struct@crate::ActiveConnection], [`Checkpoint`][struct@crate::Checkpoint], [`Device`][struct@crate::Device], [`DhcpConfig`][struct@crate::DhcpConfig], [`IPConfig`][struct@crate::IPConfig], [`Object`][struct@crate::Object], [`RemoteConnection`][struct@crate::RemoteConnection], [`WifiP2PPeer`][struct@crate::WifiP2PPeer], [`WimaxNsp`][struct@crate::WimaxNsp]
pub trait ObjectExt: 'static {
    /// Returns the [`Client`][crate::Client] instance in which object is cached.
    /// Also, if the object got removed from the client cached,
    /// this returns [`None`]. So it can be used to check whether the
    /// object is still alive.
    ///
    /// # Returns
    ///
    /// the [`Client`][crate::Client] cache in which the
    /// object can be found, or [`None`] if the object is no longer
    /// cached.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_object_get_client")]
    #[doc(alias = "get_client")]
    fn client(&self) -> Option<Client>;

    /// Gets the DBus path of the [`Object`][crate::Object].
    ///
    /// # Returns
    ///
    /// the object's path. This is the internal string used by the
    /// object, and must not be modified.
    ///
    /// Note that the D-Bus path of an NMObject never changes, even
    /// if the instance gets removed from the cache. To find out
    /// whether the object is still alive/cached, check [`client()`][Self::client()].
    #[doc(alias = "nm_object_get_path")]
    #[doc(alias = "get_path")]
    fn path(&self) -> Option<glib::GString>;

    #[doc(alias = "path")]
    fn connect_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Object>> ObjectExt for O {
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn client(&self) -> Option<Client> {
        unsafe { from_glib_none(ffi::nm_object_get_client(self.as_ref().to_glib_none().0)) }
    }

    fn path(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_object_get_path(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "path")]
    fn connect_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_path_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(
            this: *mut ffi::NMObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}
