#![warn(missing_docs)]

//! This crate contains the types and implementations that are shared between
//! all platforms that are capable of presenting a graphical user interface.

// Work around for rust-lang/rust#59368.
//! <style>
//! #macros, #macros ~ table { display: none !important; }
//! </style>

pub mod animation;
pub mod assets;
pub mod color;
pub mod components;
pub mod events;
pub mod font;
pub mod geometry;
pub mod handles;
pub mod hooks;
pub mod layout;
pub mod linalg;
pub mod macros;
pub mod physics;
pub mod prelude;
pub mod queries;
pub mod styles;
