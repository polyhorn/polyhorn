mod automaton;
mod codegen;
mod driver;
mod macros;
mod style;
mod to_tokens;
mod types;
mod value;

use automaton::{Automaton, Edge};
use codegen::Codegen;
use driver::{Driver, Error, Property};
use macros::yoyo_impl;
use style::StyleBuilder;
use to_tokens::CascoError;
use types::{Spring, Style, TransformTransition, Transition, Transitions};
use value::PropertyValue;

use proc_macro::TokenStream;

#[proc_macro]
pub fn yoyo(input: TokenStream) -> TokenStream {
    yoyo_impl(input.into()).into()
}
