use proc_macro2::{Group, Ident, Literal, TokenStream};
use std::iter::FromIterator;

mod error;
mod tokenizer;

use error::Error;
use tokenizer::{TagOpen, Token};

pub struct RegularElement {
    open: TagOpen,
    children: Vec<Element>,
}

pub enum Element {
    Regular(RegularElement),
    Literal(Literal),
    Group(Group),
}

fn build_element(open: TagOpen, remaining: &mut Vec<Token>) -> Result<Element, Error> {
    let mut children = vec![];

    while !open.is_self_closing && !remaining.is_empty() {
        match remaining.remove(0) {
            Token::TagOpen(open) => children.push(build_element(open, remaining)?),
            Token::Literal(literal) => children.push(Element::Literal(literal)),
            Token::Group(group) => children.push(Element::Group(group)),
            Token::TagClose(close) => {
                if close.is_builtin != open.is_builtin
                    || close.path.to_string() != open.path.to_string()
                {
                    return Err(Error::UnrecognizedClosingTag(open.path, close.path));
                }

                return Ok(Element::Regular(RegularElement { open, children }));
            }
        }
    }

    Ok(Element::Regular(RegularElement { open, children }))
}

fn parse(input: TokenStream) -> Result<Element, Error> {
    let mut tokens = tokenizer::parse(input)?;

    build_element(
        match tokens.remove(0) {
            Token::TagOpen(open) => open,
            _ => panic!("Expected open tag"),
        },
        &mut tokens,
    )
}

fn gen_error(
    message: &str,
    start: proc_macro2::Span,
    end: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    let mut values = vec![];
    values.push(respan(
        proc_macro2::Literal::string(message),
        proc_macro2::Span::call_site(),
    ));
    let group = values.into_iter().collect();

    let mut result = vec![];
    result.push(respan(
        proc_macro2::Ident::new("compile_error", start),
        start,
    ));
    result.push(respan(
        proc_macro2::Punct::new('!', proc_macro2::Spacing::Alone),
        proc_macro2::Span::call_site(),
    ));
    result.push(respan(
        proc_macro2::Group::new(proc_macro2::Delimiter::Brace, group),
        end,
    ));

    result.into_iter().collect()
}

fn respan<T: Into<proc_macro2::TokenTree>>(
    t: T,
    span: proc_macro2::Span,
) -> proc_macro2::TokenTree {
    let mut t = t.into();
    t.set_span(span);
    t
}

impl Element {
    pub fn as_tokenstream(&self) -> TokenStream {
        match self {
            Element::Regular(regular) => regular.as_tokenstream(),
            Element::Group(group) => quote! {
                #group.into()
            },
            Element::Literal(literal) => quote! {
                polyhorn::Element::string(#literal)
            },
        }
    }
}

impl RegularElement {
    pub fn as_tokenstream(&self) -> TokenStream {
        let path = &self.open.path;

        let children = self
            .children
            .iter()
            .map(|child| child.as_tokenstream())
            .collect::<Vec<_>>();
        let children = quote! { polyhorn::Element::fragment(polyhorn::Key::from(polyhorn::hooks::use_id!()), vec![
            #(#children),*
        ]) };

        if self.open.is_builtin {
            let mut error = None;

            for prop in &self.open.props {
                if prop.name.to_string() != "ref" {
                    error = Some(gen_error(
                        &format!(
                            "Prop `{}` does not exist on built-in type: `{}`.",
                            prop.name, path
                        ),
                        prop.name.span(),
                        prop.name.span(),
                    ));
                }
            }

            let reference = if let Some(reference) = self
                .open
                .props
                .iter()
                .find(|item| item.name.to_string() == "ref")
            {
                let value = reference.value.clone().unwrap();

                quote! {#[allow(unused_braces)]
                #(#value)*}
            } else {
                quote!(None)
            };

            (quote! {{
                #error
                polyhorn::Element::builtin(polyhorn::Key::from(polyhorn::hooks::use_id!()), #path, #children, #reference)
            }})
            .into()
        } else {
            let mut props = vec![];

            let mut key = None;

            for prop in &self.open.props {
                let name = &prop.name;
                let value = &prop.value;

                if prop.name == "key" {
                    key = value.clone();

                    continue;
                }

                let value = value
                    .clone()
                    .unwrap_or(vec![Ident::new("true", name.span()).into()]);

                props.push(quote! {
                    #name: #(#value)*.into(),
                });
            }

            let props = TokenStream::from_iter(props);

            let dots = match self.open.is_default {
                true => quote! { ..Default::default() },
                false => quote! {},
            };

            let key = key
                .map(|key| {
                    quote! { polyhorn::Key::new(
                        #[allow(unused_braces)]
                        #(#key)*
                    ) }
                })
                .unwrap_or_else(|| quote! { polyhorn::Key::from(polyhorn::hooks::use_id!()) });

            (quote! {
                polyhorn::Element::new(#key, #path {
                    #props
                    #dots
                }.into(), #children)
            })
            .into()
        }
    }
}

use quote::quote;

pub fn poly(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse(proc_macro2::TokenStream::from(input)) {
        Ok(element) => element.as_tokenstream().into(),
        Err(error) => gen_error(&error.message(), error.start(), error.end()).into(),
    }
}
