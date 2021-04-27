// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::CellArea;
use crate::CellLayout;
use crate::CellRenderer;
use crate::Orientable;
use crate::Orientation;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox, ffi::GtkCellAreaBoxClass>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    #[doc(alias = "gtk_cell_area_box_new")]
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe { CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).unsafe_cast() }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CellAreaBoxBuilder {
    spacing: Option<i32>,
    focus_cell: Option<CellRenderer>,
    orientation: Option<Orientation>,
}

impl CellAreaBoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CellAreaBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref focus_cell) = self.focus_cell {
            properties.push(("focus-cell", focus_cell));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<CellAreaBox>(&properties).expect("object new");
        ret
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn focus_cell<P: IsA<CellRenderer>>(mut self, focus_cell: &P) -> Self {
        self.focus_cell = Some(focus_cell.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_CELL_AREA_BOX: Option<&CellAreaBox> = None;

pub trait CellAreaBoxExt: 'static {
    #[doc(alias = "gtk_cell_area_box_get_spacing")]
    fn spacing(&self) -> i32;

    #[doc(alias = "gtk_cell_area_box_pack_end")]
    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool);

    #[doc(alias = "gtk_cell_area_box_pack_start")]
    fn pack_start<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    );

    #[doc(alias = "gtk_cell_area_box_set_spacing")]
    fn set_spacing(&self, spacing: i32);

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellAreaBox>> CellAreaBoxExt for O {
    fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_cell_area_box_get_spacing(self.as_ref().to_glib_none().0) }
    }

    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    fn pack_start<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellAreaBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellAreaBox>,
        {
            let f: &F = &*(f as *const F);
            f(&CellAreaBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellAreaBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellAreaBox")
    }
}
