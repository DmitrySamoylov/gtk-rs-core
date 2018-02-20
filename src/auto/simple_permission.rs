// This file was generated by gir (https://github.com/gtk-rs/gir @ 72ba992)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Permission;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SimplePermission(Object<ffi::GSimplePermission>): Permission;

    match fn {
        get_type => || ffi::g_simple_permission_get_type(),
    }
}

impl SimplePermission {
    pub fn new(allowed: bool) -> SimplePermission {
        unsafe {
            Permission::from_glib_full(ffi::g_simple_permission_new(allowed.to_glib())).downcast_unchecked()
        }
    }
}
