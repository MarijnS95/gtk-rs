// Take a look at the license at the top of the repository in the LICENSE file.

//! # ATK bindings
//!
//! This library contains safe Rust bindings for [ATK](https://developer.gnome.org/atk/). It's
//! a part of [Gtk-rs](https://gtk-rs.org/).

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use glib;

#[macro_use]
mod rt;

mod auto;

pub use crate::auto::*;

pub mod prelude;

pub use crate::attribute::Attribute;
pub use crate::attribute_set::AttributeSet;
pub use crate::text_rectangle::TextRectangle;

mod attribute;
mod attribute_set;
mod editable_text;
mod table;
mod text_rectangle;
