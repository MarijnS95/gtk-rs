// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use Orientable;
#[cfg(feature = "v3_10")]
use Stack;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct StackSwitcher(Object<ffi::GtkStackSwitcher>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_stack_switcher_get_type(),
    }
}

impl StackSwitcher {
    #[cfg(feature = "v3_10")]
    pub fn new() -> StackSwitcher {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_switcher_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(ffi::gtk_stack_switcher_get_stack(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_stack(&self, stack: Option<&Stack>) {
        unsafe {
            ffi::gtk_stack_switcher_set_stack(self.to_glib_none().0, stack.to_glib_none().0);
        }
    }
}
