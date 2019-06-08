// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Adjustment;
use Align;
use Bin;
use Buildable;
use Button;
use Container;
use IconSize;
use Orientable;
use PositionType;
use ReliefStyle;
use ResizeMode;
use Widget;
use gdk;
use glib;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
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
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ScaleButton(Object<gtk_sys::GtkScaleButton, gtk_sys::GtkScaleButtonClass, ScaleButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    pub fn new(size: IconSize, min: f64, max: f64, step: f64, icons: &[&str]) -> ScaleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scale_button_new(size.to_glib(), min, max, step, icons.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub struct ScaleButtonBuilder {
    adjustment: Option<Adjustment>,
    icons: Option<Vec<String>>,
    size: Option<IconSize>,
    value: Option<f64>,
    always_show_image: Option<bool>,
    image: Option<Widget>,
    image_position: Option<PositionType>,
    label: Option<String>,
    relief: Option<ReliefStyle>,
    use_underline: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ScaleButtonBuilder {
    pub fn new() -> Self {
        Self {
            adjustment: None,
            icons: None,
            size: None,
            value: None,
            always_show_image: None,
            image: None,
            image_position: None,
            label: None,
            relief: None,
            use_underline: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> ScaleButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref icons) = self.icons {
            properties.push(("icons", icons));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref always_show_image) = self.always_show_image {
            properties.push(("always-show-image", always_show_image));
        }
        if let Some(ref image) = self.image {
            properties.push(("image", image));
        }
        if let Some(ref image_position) = self.image_position {
            properties.push(("image-position", image_position));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref relief) = self.relief {
            properties.push(("relief", relief));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(ScaleButton::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn adjustment(mut self, adjustment: &Adjustment) -> Self {
        self.adjustment = Some(adjustment.clone());
        self
    }

    pub fn icons(mut self, icons: Vec<String>) -> Self {
        self.icons = Some(icons);
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn always_show_image(mut self, always_show_image: bool) -> Self {
        self.always_show_image = Some(always_show_image);
        self
    }

    pub fn image(mut self, image: &Widget) -> Self {
        self.image = Some(image.clone());
        self
    }

    pub fn image_position(mut self, image_position: PositionType) -> Self {
        self.image_position = Some(image_position);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn relief(mut self, relief: ReliefStyle) -> Self {
        self.relief = Some(relief);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_SCALE_BUTTON: Option<&ScaleButton> = None;

pub trait ScaleButtonExt: 'static {
    fn get_adjustment(&self) -> Adjustment;

    fn get_minus_button(&self) -> Option<Button>;

    fn get_plus_button(&self) -> Option<Button>;

    fn get_popup(&self) -> Option<Widget>;

    fn get_value(&self) -> f64;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_icons(&self, icons: &[&str]);

    fn set_value(&self, value: f64);

    fn get_property_icons(&self) -> Vec<GString>;

    fn get_property_size(&self) -> IconSize;

    fn set_property_size(&self, size: IconSize);

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self);

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScaleButton>> ScaleButtonExt for O {
    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(gtk_sys::gtk_scale_button_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_minus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(gtk_sys::gtk_scale_button_get_minus_button(self.as_ref().to_glib_none().0))
        }
    }

    fn get_plus_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(gtk_sys::gtk_scale_button_get_plus_button(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_scale_button_get_popup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_scale_button_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            gtk_sys::gtk_scale_button_set_adjustment(self.as_ref().to_glib_none().0, adjustment.as_ref().to_glib_none().0);
        }
    }

    fn set_icons(&self, icons: &[&str]) {
        unsafe {
            gtk_sys::gtk_scale_button_set_icons(self.as_ref().to_glib_none().0, icons.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_scale_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn get_property_icons(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"icons\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<IconSize as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_size(&self, size: IconSize) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"size\0".as_ptr() as *const _, Value::from(&size).to_glib_none().0);
        }
    }

    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popdown_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popdown\0".as_ptr() as *const _,
                Some(transmute(popdown_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_popdown(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("popdown", &[]).unwrap() };
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popup\0".as_ptr() as *const _,
                Some(transmute(popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_popup(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("popup", &[]).unwrap() };
    }

    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P, f64) + 'static>(this: *mut gtk_sys::GtkScaleButton, value: libc::c_double, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast(), value)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"value-changed\0".as_ptr() as *const _,
                Some(transmute(value_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icons_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icons\0".as_ptr() as *const _,
                Some(transmute(notify_icons_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScaleButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ScaleButton>
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ScaleButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScaleButton")
    }
}
