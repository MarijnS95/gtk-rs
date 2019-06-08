// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_24", feature = "dox"))]
use FontChooserLevel;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FontChooser(Interface<gtk_sys::GtkFontChooser>);

    match fn {
        get_type => || gtk_sys::gtk_font_chooser_get_type(),
    }
}

pub const NONE_FONT_CHOOSER: Option<&FontChooser> = None;

pub trait FontChooserExt: 'static {
    fn get_font(&self) -> Option<GString>;

    fn get_font_desc(&self) -> Option<pango::FontDescription>;

    fn get_font_face(&self) -> Option<pango::FontFace>;

    fn get_font_family(&self) -> Option<pango::FontFamily>;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_font_features(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap>;

    fn get_font_size(&self) -> i32;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_language(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_level(&self) -> FontChooserLevel;

    fn get_preview_text(&self) -> Option<GString>;

    fn get_show_preview_entry(&self) -> bool;

    fn set_filter_func(&self, filter: Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>);

    fn set_font(&self, fontname: &str);

    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<P: IsA<pango::FontMap>>(&self, fontmap: Option<&P>);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_language(&self, language: &str);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_level(&self, level: FontChooserLevel);

    fn set_preview_text(&self, text: &str);

    fn set_show_preview_entry(&self, show_preview_entry: bool);

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_font_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FontChooser>> FontChooserExt for O {
    fn get_font(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_font(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_font_desc(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(gtk_sys::gtk_font_chooser_get_font_face(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(gtk_sys::gtk_font_chooser_get_font_family(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_font_features(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_font_features(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_font_map(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_size(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_font_chooser_get_font_size(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_language(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_language(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_level(&self) -> FontChooserLevel {
        unsafe {
            from_glib(gtk_sys::gtk_font_chooser_get_level(self.as_ref().to_glib_none().0))
        }
    }

    fn get_preview_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_font_chooser_get_preview_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_chooser_get_show_preview_entry(self.as_ref().to_glib_none().0))
        }
    }

    fn set_filter_func(&self, filter: Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>) {
        let filter_data: Box_<Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>> = Box::new(filter);
        unsafe extern "C" fn filter_func(family: *const pango_sys::PangoFontFamily, face: *const pango_sys::PangoFontFace, data: glib_sys::gpointer) -> glib_sys::gboolean {
            let family = from_glib_borrow(family);
            let face = from_glib_borrow(face);
            let callback: &Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>> = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&family, &face)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter = if filter_data.is_some() { Some(filter_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>> = filter_data;
        unsafe {
            gtk_sys::gtk_font_chooser_set_filter_func(self.as_ref().to_glib_none().0, filter, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_font(&self, fontname: &str) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_font(self.as_ref().to_glib_none().0, fontname.to_glib_none().0);
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_font_desc(self.as_ref().to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<P: IsA<pango::FontMap>>(&self, fontmap: Option<&P>) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_font_map(self.as_ref().to_glib_none().0, fontmap.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_language(&self, language: &str) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_language(self.as_ref().to_glib_none().0, language.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_level(&self, level: FontChooserLevel) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_level(self.as_ref().to_glib_none().0, level.to_glib());
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_preview_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            gtk_sys::gtk_font_chooser_set_show_preview_entry(self.as_ref().to_glib_none().0, show_preview_entry.to_glib());
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_activated_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut gtk_sys::GtkFontChooser, fontname: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(fontname))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"font-activated\0".as_ptr() as *const _,
                Some(transmute(font_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::font\0".as_ptr() as *const _,
                Some(transmute(notify_font_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute(notify_font_desc_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_font_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_features_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::font-features\0".as_ptr() as *const _,
                Some(transmute(notify_font_features_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::language\0".as_ptr() as *const _,
                Some(transmute(notify_language_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::level\0".as_ptr() as *const _,
                Some(transmute(notify_level_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_preview_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::preview-text\0".as_ptr() as *const _,
                Some(transmute(notify_preview_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_preview_entry_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFontChooser, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FontChooser>
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-preview-entry\0".as_ptr() as *const _,
                Some(transmute(notify_show_preview_entry_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FontChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooser")
    }
}
