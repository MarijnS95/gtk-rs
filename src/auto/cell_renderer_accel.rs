// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use TreePath;
use ffi;
use gdk;
use gdk_ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).downcast_unchecked()
        }
    }

    pub fn connect_accel_cleared<F: Fn(&CellRendererAccel, TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&CellRendererAccel, TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-cleared",
                transmute(accel_cleared_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_accel_edited<F: Fn(&CellRendererAccel, TreePath, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&CellRendererAccel, TreePath, u32, gdk::ModifierType, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-edited",
                transmute(accel_edited_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accel_cleared_trampoline(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&CellRendererAccel, TreePath) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&from_glib_none(this), path)
}

unsafe extern "C" fn accel_edited_trampoline(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_ffi::GdkModifierType, hardware_keycode: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&CellRendererAccel, TreePath, u32, gdk::ModifierType, u32) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&from_glib_none(this), path, accel_key, from_glib(accel_mods), hardware_keycode)
}
