use inflections::Inflect;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

use super::{Edge, Style};

pub struct Codegen<'a> {
    variants: &'a [(&'a Edge, Style)],
}

impl<'a> Codegen<'a> {
    pub fn new(variants: &'a [(&'a Edge, Style)]) -> Codegen<'a> {
        Codegen { variants }
    }

    pub fn generate_style(style: &Style) -> TokenStream {
        let opacity = &style.opacity;
        let transform = &style.transform;

        quote! {
            yoyo::Style {
                opacity: #opacity,
                transform: [#(#transform),*],
            }
        }
    }

    pub fn generate_transitions(style: &Style) -> TokenStream {
        let opacity = &style.transitions.opacity;
        let transform = &style.transitions.transform;

        quote! {
            yoyo::Transitions {
                opacity: #opacity,
                transform: #transform,
            }
        }
    }

    pub fn generate_state(name: Option<&str>) -> TokenStream {
        match name {
            Some(name) => {
                let ident = Ident::new(&name.to_pascal_case(), Span::call_site());
                quote! { Self::#ident }
            }
            None => quote! { _ },
        }
    }

    pub fn generate(self, ty: Ident) -> TokenStream {
        let mut names = HashSet::new();

        names.insert("initial");

        for (edge, _) in self.variants {
            if let Some(from) = edge.from() {
                names.insert(&from);
            }

            if let Some(to) = edge.to() {
                names.insert(&to);
            }
        }

        let press = match names.contains("press") {
            true => quote! { Some(Self::Press) },
            false => quote! { None },
        };

        let exit = match names.contains("exit") {
            true => quote! { Some(Self::Exit) },
            false => quote! { None },
        };

        // TODO: it would be very helpful if we still had the Spans here.
        let names = names
            .into_iter()
            .map(|name| Ident::new(&name.to_pascal_case(), Span::call_site()))
            .collect::<Vec<_>>();

        let match_style = self
            .variants
            .into_iter()
            .filter(|(edge, _)| edge.from().is_none())
            .map(|(edge, style)| {
                let variant = Self::generate_state(edge.to());

                let style = Self::generate_style(style);

                quote! {
                    #variant => #style,
                }
            })
            .collect::<Vec<_>>();

        let match_transition = self.variants.into_iter().map(|(edge, style)| {
            let from = Self::generate_state(edge.from());
            let to = Self::generate_state(edge.to());

            let transitions = Self::generate_transitions(style);

            quote! {
                (#from, #to) => #transitions,
            }
        });

        quote! {
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            enum #ty {
                #(#names,)*
            }

            impl yoyo::Variants for #ty {
                fn initial() -> Self {
                    Self::Initial
                }

                fn press() -> Option<Self> {
                    #press
                }

                fn exit() -> Option<Self> {
                    #exit
                }

                fn style(&self) -> yoyo::Style {
                    match self {
                        #(#match_style)*
                    }
                }

                fn transitions(from: &Self, to: &Self) -> yoyo::Transitions {
                    match (from, to) {
                        #(#match_transition)*
                    }
                }
            }
        }
    }
}
