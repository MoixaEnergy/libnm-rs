#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate libc;

extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;

extern crate gio;

extern crate nm_sys as ffi;

pub use glib::Error;

mod auto;
pub use auto::functions::*;
pub use auto::*;

mod array;

pub mod client;

pub mod functions;
pub use functions::*;

pub mod prelude {
    pub use glib::prelude::*;

    pub use auto::traits::*;
}

pub use prelude::*;
