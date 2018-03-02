// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Window;
use ffi;
use gdk;
use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct MountOperation(Object<ffi::GtkMountOperation, ffi::GtkMountOperationClass>): [
        gio::MountOperation => gio_ffi::GMountOperation,
    ];

    match fn {
        get_type => || ffi::gtk_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub fn new<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(parent: Q) -> MountOperation {
        assert_initialized_main_thread!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            gio::MountOperation::from_glib_full(ffi::gtk_mount_operation_new(parent.0)).downcast_unchecked()
        }
    }
}

pub trait MountOperationExt {
    fn get_parent(&self) -> Option<Window>;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn is_showing(&self) -> bool;

    fn set_parent<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q);

    fn set_screen(&self, screen: &gdk::Screen);

    fn get_property_is_showing(&self) -> bool;

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation> + IsA<glib::object::Object>> MountOperationExt for O {
    fn get_parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_parent(self.to_glib_none().0))
        }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_screen(self.to_glib_none().0))
        }
    }

    fn is_showing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_mount_operation_is_showing(self.to_glib_none().0))
        }
    }

    fn set_parent<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            ffi::gtk_mount_operation_set_parent(self.to_glib_none().0, parent.0);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_mount_operation_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn get_property_is_showing(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-showing".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-showing",
                transmute(notify_is_showing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent",
                transmute(notify_parent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::screen",
                transmute(notify_screen_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_is_showing_trampoline<P>(this: *mut ffi::GtkMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_trampoline<P>(this: *mut ffi::GtkMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_screen_trampoline<P>(this: *mut ffi::GtkMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}
