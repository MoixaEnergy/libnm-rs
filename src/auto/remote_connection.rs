// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
#[cfg(feature = "futures")]
use futures_core;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::GString;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Connection;
use Error;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use SettingsConnectionFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use SettingsUpdate2Flags;

glib_wrapper! {
    pub struct RemoteConnection(Object<ffi::NMRemoteConnection, ffi::NMRemoteConnectionClass>): Connection;

    match fn {
        get_type => || ffi::nm_remote_connection_get_type(),
    }
}

pub trait RemoteConnectionExt: 'static {
    fn commit_changes<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        save_to_disk: bool,
        cancellable: P,
    ) -> Result<(), Error>;

    fn commit_changes_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        save_to_disk: bool,
        cancellable: P,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn commit_changes_async_future(
        &self,
        save_to_disk: bool,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone;

    fn delete<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        cancellable: P,
    ) -> Result<(), Error>;

    fn delete_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        cancellable: P,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn delete_async_future(
        &self,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_filename(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_flags(&self) -> SettingsConnectionFlags;

    fn get_secrets<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        setting_name: &str,
        cancellable: P,
    ) -> Result<glib::Variant, Error>;

    fn get_secrets_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
    >(
        &self,
        setting_name: &str,
        cancellable: P,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn get_secrets_async_future(
        &self,
        setting_name: &str,
    ) -> Box_<futures_core::Future<Item = (Self, glib::Variant), Error = (Self, Error)>>
    where
        Self: Sized + Clone;

    fn get_unsaved(&self) -> bool;

    fn get_visible(&self) -> bool;

    fn save<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn save_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        cancellable: P,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn save_async_future(
        &self,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn update2<
        'a,
        'b,
        'c,
        P: Into<Option<&'a glib::Variant>>,
        Q: Into<Option<&'b glib::Variant>>,
        R: Into<Option<&'c gio::Cancellable>>,
        S: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
    >(
        &self,
        settings: P,
        flags: SettingsUpdate2Flags,
        args: Q,
        cancellable: R,
        callback: S,
    );

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn update2_future<
        'a,
        'b,
        P: Into<Option<&'a glib::Variant>>,
        Q: Into<Option<&'b glib::Variant>>,
    >(
        &self,
        settings: P,
        flags: SettingsUpdate2Flags,
        args: Q,
    ) -> Box_<futures_core::Future<Item = (Self, glib::Variant), Error = (Self, Error)>>
    where
        Self: Sized + Clone;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_unsaved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RemoteConnection>> RemoteConnectionExt for O {
    fn commit_changes<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        save_to_disk: bool,
        cancellable: P,
    ) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_commit_changes(
                self.to_glib_none().0,
                save_to_disk.to_glib(),
                cancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn commit_changes_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        save_to_disk: bool,
        cancellable: P,
        callback: Q,
    ) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn commit_changes_async_trampoline<
            Q: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_ffi::GObject,
            res: *mut gio_ffi::GAsyncResult,
            user_data: glib_ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_commit_changes_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = commit_changes_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_commit_changes_async(
                self.to_glib_none().0,
                save_to_disk.to_glib(),
                cancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn commit_changes_async_future(
        &self,
        save_to_disk: bool,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone,
    {
        use fragile::Fragile;
        use gio::GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.commit_changes_async(save_to_disk, Some(&cancellable), move |res| {
                let obj = obj_clone.into_inner();
                let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn delete<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        cancellable: P,
    ) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_delete(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn delete_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        cancellable: P,
        callback: Q,
    ) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn delete_async_trampoline<
            Q: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_ffi::GObject,
            res: *mut gio_ffi::GAsyncResult,
            user_data: glib_ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_delete_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = delete_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_delete_async(
                self.to_glib_none().0,
                cancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn delete_async_future(
        &self,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone,
    {
        use fragile::Fragile;
        use gio::GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.delete_async(Some(&cancellable), move |res| {
                let obj = obj_clone.into_inner();
                let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::nm_remote_connection_get_filename(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_flags(&self) -> SettingsConnectionFlags {
        unsafe { from_glib(ffi::nm_remote_connection_get_flags(self.to_glib_none().0)) }
    }

    fn get_secrets<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        setting_name: &str,
        cancellable: P,
    ) -> Result<glib::Variant, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets(
                self.to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_secrets_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
    >(
        &self,
        setting_name: &str,
        cancellable: P,
        callback: Q,
    ) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_secrets_async_trampoline<
            Q: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_ffi::GObject,
            res: *mut gio_ffi::GAsyncResult,
            user_data: glib_ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_secrets_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_get_secrets_async(
                self.to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn get_secrets_async_future(
        &self,
        setting_name: &str,
    ) -> Box_<futures_core::Future<Item = (Self, glib::Variant), Error = (Self, Error)>>
    where
        Self: Sized + Clone,
    {
        use fragile::Fragile;
        use gio::GioFuture;

        let setting_name = String::from(setting_name);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.get_secrets_async(&setting_name, Some(&cancellable), move |res| {
                let obj = obj_clone.into_inner();
                let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn get_unsaved(&self) -> bool {
        unsafe { from_glib(ffi::nm_remote_connection_get_unsaved(self.to_glib_none().0)) }
    }

    fn get_visible(&self) -> bool {
        unsafe { from_glib(ffi::nm_remote_connection_get_visible(self.to_glib_none().0)) }
    }

    fn save<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_save(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn save_async<
        'a,
        P: Into<Option<&'a gio::Cancellable>>,
        Q: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        cancellable: P,
        callback: Q,
    ) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn save_async_trampoline<
            Q: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_ffi::GObject,
            res: *mut gio_ffi::GAsyncResult,
            user_data: glib_ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_save_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = save_async_trampoline::<Q>;
        unsafe {
            ffi::nm_remote_connection_save_async(
                self.to_glib_none().0,
                cancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn save_async_future(
        &self,
    ) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>
    where
        Self: Sized + Clone,
    {
        use fragile::Fragile;
        use gio::GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.save_async(Some(&cancellable), move |res| {
                let obj = obj_clone.into_inner();
                let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn update2<
        'a,
        'b,
        'c,
        P: Into<Option<&'a glib::Variant>>,
        Q: Into<Option<&'b glib::Variant>>,
        R: Into<Option<&'c gio::Cancellable>>,
        S: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
    >(
        &self,
        settings: P,
        flags: SettingsUpdate2Flags,
        args: Q,
        cancellable: R,
        callback: S,
    ) {
        let settings = settings.into();
        let settings = settings.to_glib_none();
        let args = args.into();
        let args = args.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<S>> = Box::new(Box::new(callback));
        unsafe extern "C" fn update2_trampoline<
            S: FnOnce(Result<glib::Variant, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_ffi::GObject,
            res: *mut gio_ffi::GAsyncResult,
            user_data: glib_ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::nm_remote_connection_update2_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Box<S>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = update2_trampoline::<S>;
        unsafe {
            ffi::nm_remote_connection_update2(
                self.to_glib_none().0,
                settings.0,
                flags.to_glib(),
                args.0,
                cancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn update2_future<
        'a,
        'b,
        P: Into<Option<&'a glib::Variant>>,
        Q: Into<Option<&'b glib::Variant>>,
    >(
        &self,
        settings: P,
        flags: SettingsUpdate2Flags,
        args: Q,
    ) -> Box_<futures_core::Future<Item = (Self, glib::Variant), Error = (Self, Error)>>
    where
        Self: Sized + Clone,
    {
        use fragile::Fragile;
        use gio::GioFuture;

        let settings = settings.into();
        let settings = settings.map(ToOwned::to_owned);
        let args = args.into();
        let args = args.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.update2(
                settings.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                args.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"notify::filename\0".as_ptr() as *const _,
                transmute(notify_filename_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                transmute(notify_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_unsaved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"notify::unsaved\0".as_ptr() as *const _,
                transmute(notify_unsaved_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                transmute(notify_visible_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_filename_trampoline<P>(
    this: *mut ffi::NMRemoteConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<RemoteConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn notify_flags_trampoline<P>(
    this: *mut ffi::NMRemoteConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<RemoteConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_unsaved_trampoline<P>(
    this: *mut ffi::NMRemoteConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<RemoteConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_trampoline<P>(
    this: *mut ffi::NMRemoteConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<RemoteConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RemoteConnection::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for RemoteConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RemoteConnection")
    }
}
