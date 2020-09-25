#![warn(missing_docs)]

//! Crate for parsing CSS-like structures from the input of a procedural macro.

pub mod cascade;
pub mod concrete;
pub mod domain;
pub mod lexer;
pub mod stream;

pub mod proc_macro2;
mod stylesheet;

pub use stylesheet::{Driver, Error, Item, StyleSheet};
