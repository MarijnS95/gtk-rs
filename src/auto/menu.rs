// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AccelGroup;
use Buildable;
use Container;
use MenuItem;
use MenuShell;
use ScrollType;
use Widget;
use gdk;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Menu(Object<gtk_sys::GtkMenu, gtk_sys::GtkMenuClass, MenuClass>) @extends MenuShell, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_new()).unsafe_cast()
        }
    }

    pub fn new_from_model<P: IsA<gio::MenuModel>>(model: &P) -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_new_from_model(model.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn get_for_attach_widget<P: IsA<Widget>>(widget: &P) -> Vec<Widget> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_menu_get_for_attach_widget(widget.as_ref().to_glib_none().0))
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MENU: Option<&Menu> = None;

pub trait GtkMenuExt: 'static {
    fn attach<P: IsA<Widget>>(&self, child: &P, left_attach: u32, right_attach: u32, top_attach: u32, bottom_attach: u32);

    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>);

    fn detach(&self);

    fn get_accel_group(&self) -> Option<AccelGroup>;

    fn get_accel_path(&self) -> Option<GString>;

    fn get_active(&self) -> Option<Widget>;

    fn get_attach_widget(&self) -> Option<Widget>;

    fn get_monitor(&self) -> i32;

    fn get_reserve_toggle_size(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn place_on_monitor(&self, monitor: &gdk::Monitor);

    fn popdown(&self);

    //#[cfg_attr(feature = "v3_22", deprecated)]
    //fn popup<P: IsA<Widget>, Q: IsA<Widget>>(&self, parent_menu_shell: Option<&P>, parent_menu_item: Option<&Q>, func: Option<Box<dyn FnOnce(&Menu, i32, i32, bool) + 'static>>, button: u32, activate_time: u32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_pointer(&self, trigger_event: Option<&gdk::Event>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_rect<P: IsA<gdk::Window>>(&self, rect_window: &P, rect: &gdk::Rectangle, rect_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>);

    //#[cfg_attr(feature = "v3_22", deprecated)]
    //fn popup_for_device<P: IsA<Widget>, Q: IsA<Widget>>(&self, device: Option<&gdk::Device>, parent_menu_shell: Option<&P>, parent_menu_item: Option<&Q>, func: Option<Box<dyn Fn(&Menu, i32, i32, bool) + 'static>>, button: u32, activate_time: u32);

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn reposition(&self);

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>);

    fn set_accel_path(&self, accel_path: Option<&str>);

    fn set_active(&self, index: u32);

    fn set_monitor(&self, monitor_num: i32);

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool);

    fn set_screen(&self, screen: Option<&gdk::Screen>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_anchor_hints(&self) -> gdk::AnchorHints;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_anchor_hints(&self, anchor_hints: gdk::AnchorHints);

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_menu_type_hint(&self) -> gdk::WindowTypeHint;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_menu_type_hint(&self, menu_type_hint: gdk::WindowTypeHint);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_rect_anchor_dx(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_rect_anchor_dy(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32);

    fn get_item_bottom_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32;

    fn set_item_bottom_attach<T: IsA<MenuItem>>(&self, item: &T, bottom_attach: i32);

    fn get_item_left_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32;

    fn set_item_left_attach<T: IsA<MenuItem>>(&self, item: &T, left_attach: i32);

    fn get_item_right_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32;

    fn set_item_right_attach<T: IsA<MenuItem>>(&self, item: &T, right_attach: i32);

    fn get_item_top_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32;

    fn set_item_top_attach<T: IsA<MenuItem>>(&self, item: &T, top_attach: i32);

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_scroll(&self, scroll_type: ScrollType);

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Menu>> GtkMenuExt for O {
    fn attach<P: IsA<Widget>>(&self, child: &P, left_attach: u32, right_attach: u32, top_attach: u32, bottom_attach: u32) {
        unsafe {
            gtk_sys::gtk_menu_attach(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, left_attach, right_attach, top_attach, bottom_attach);
        }
    }

    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_attach_to_widget() }
    //}

    fn detach(&self) {
        unsafe {
            gtk_sys::gtk_menu_detach(self.as_ref().to_glib_none().0);
        }
    }

    fn get_accel_group(&self) -> Option<AccelGroup> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_group(self.as_ref().to_glib_none().0))
        }
    }

    fn get_accel_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_path(self.as_ref().to_glib_none().0))
        }
    }

    fn get_active(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_active(self.as_ref().to_glib_none().0))
        }
    }

    fn get_attach_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_attach_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_monitor(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_menu_get_monitor(self.as_ref().to_glib_none().0)
        }
    }

    fn get_reserve_toggle_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_get_reserve_toggle_size(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn place_on_monitor(&self, monitor: &gdk::Monitor) {
        unsafe {
            gtk_sys::gtk_menu_place_on_monitor(self.as_ref().to_glib_none().0, monitor.to_glib_none().0);
        }
    }

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_menu_popdown(self.as_ref().to_glib_none().0);
        }
    }

    //fn popup<P: IsA<Widget>, Q: IsA<Widget>>(&self, parent_menu_shell: Option<&P>, parent_menu_item: Option<&Q>, func: Option<Box<dyn FnOnce(&Menu, i32, i32, bool) + 'static>>, button: u32, activate_time: u32) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_popup() }
    //}

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_pointer(&self, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_pointer(self.as_ref().to_glib_none().0, trigger_event.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_rect<P: IsA<gdk::Window>>(&self, rect_window: &P, rect: &gdk::Rectangle, rect_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_rect(self.as_ref().to_glib_none().0, rect_window.as_ref().to_glib_none().0, rect.to_glib_none().0, rect_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: gdk::Gravity, menu_anchor: gdk::Gravity, trigger_event: Option<&gdk::Event>) {
        unsafe {
            gtk_sys::gtk_menu_popup_at_widget(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, widget_anchor.to_glib(), menu_anchor.to_glib(), trigger_event.to_glib_none().0);
        }
    }

    //fn popup_for_device<P: IsA<Widget>, Q: IsA<Widget>>(&self, device: Option<&gdk::Device>, parent_menu_shell: Option<&P>, parent_menu_item: Option<&Q>, func: Option<Box<dyn Fn(&Menu, i32, i32, bool) + 'static>>, button: u32, activate_time: u32) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_popup_for_device() }
    //}

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_menu_reorder_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    fn reposition(&self) {
        unsafe {
            gtk_sys::gtk_menu_reposition(self.as_ref().to_glib_none().0);
        }
    }

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_group(self.as_ref().to_glib_none().0, accel_group.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_path(self.as_ref().to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    fn set_active(&self, index: u32) {
        unsafe {
            gtk_sys::gtk_menu_set_active(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set_monitor(&self, monitor_num: i32) {
        unsafe {
            gtk_sys::gtk_menu_set_monitor(self.as_ref().to_glib_none().0, monitor_num);
        }
    }

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool) {
        unsafe {
            gtk_sys::gtk_menu_set_reserve_toggle_size(self.as_ref().to_glib_none().0, reserve_toggle_size.to_glib());
        }
    }

    fn set_screen(&self, screen: Option<&gdk::Screen>) {
        unsafe {
            gtk_sys::gtk_menu_set_screen(self.as_ref().to_glib_none().0, screen.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_anchor_hints(&self) -> gdk::AnchorHints {
        unsafe {
            let mut value = Value::from_type(<gdk::AnchorHints as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_anchor_hints(&self, anchor_hints: gdk::AnchorHints) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, Value::from(&anchor_hints).to_glib_none().0);
        }
    }

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"attach-widget\0".as_ptr() as *const _, Value::from(attach_widget).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_menu_type_hint(&self) -> gdk::WindowTypeHint {
        unsafe {
            let mut value = Value::from_type(<gdk::WindowTypeHint as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_menu_type_hint(&self, menu_type_hint: gdk::WindowTypeHint) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, Value::from(&menu_type_hint).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_rect_anchor_dx(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, Value::from(&rect_anchor_dx).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_property_rect_anchor_dy(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, Value::from(&rect_anchor_dy).to_glib_none().0);
        }
    }

    fn get_item_bottom_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"bottom-attach\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_bottom_attach<T: IsA<MenuItem>>(&self, item: &T, bottom_attach: i32) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"bottom-attach\0".as_ptr() as *const _, Value::from(&bottom_attach).to_glib_none().0);
        }
    }

    fn get_item_left_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"left-attach\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_left_attach<T: IsA<MenuItem>>(&self, item: &T, left_attach: i32) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"left-attach\0".as_ptr() as *const _, Value::from(&left_attach).to_glib_none().0);
        }
    }

    fn get_item_right_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"right-attach\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_right_attach<T: IsA<MenuItem>>(&self, item: &T, right_attach: i32) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"right-attach\0".as_ptr() as *const _, Value::from(&right_attach).to_glib_none().0);
        }
    }

    fn get_item_top_attach<T: IsA<MenuItem>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"top-attach\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_top_attach<T: IsA<MenuItem>>(&self, item: &T, top_attach: i32) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(self.to_glib_none().0 as *mut gtk_sys::GtkContainer, item.to_glib_none().0 as *mut _, b"top-attach\0".as_ptr() as *const _, Value::from(&top_attach).to_glib_none().0);
        }
    }

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-scroll\0".as_ptr() as *const _,
                Some(transmute(move_scroll_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_scroll(&self, scroll_type: ScrollType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-scroll", &[&scroll_type]).unwrap() };
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-group\0".as_ptr() as *const _,
                Some(transmute(notify_accel_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-path\0".as_ptr() as *const _,
                Some(transmute(notify_accel_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor-hints\0".as_ptr() as *const _,
                Some(transmute(notify_anchor_hints_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attach-widget\0".as_ptr() as *const _,
                Some(transmute(notify_attach_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-type-hint\0".as_ptr() as *const _,
                Some(transmute(notify_menu_type_hint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitor\0".as_ptr() as *const _,
                Some(transmute(notify_monitor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dx\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dx_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dy\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reserve-toggle-size\0".as_ptr() as *const _,
                Some(transmute(notify_reserve_toggle_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn move_scroll_trampoline<P, F: Fn(&P, ScrollType) + 'static>(this: *mut gtk_sys::GtkMenu, scroll_type: gtk_sys::GtkScrollType, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast(), from_glib(scroll_type))
}

unsafe extern "C" fn notify_accel_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_anchor_hints_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_attach_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_menu_type_hint_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_monitor_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_rect_anchor_dx_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_rect_anchor_dy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_reserve_toggle_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
