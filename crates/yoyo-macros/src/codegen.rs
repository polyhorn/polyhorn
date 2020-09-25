use inflections::Inflect;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

use super::{Edge, Style, Transition};

pub struct Codegen<'a> {
    variants: &'a [(Edge<'a>, Style)],
}

impl<'a> Codegen<'a> {
    pub fn new(variants: &'a [(Edge<'a>, Style)]) -> Codegen<'a> {
        Codegen { variants }
    }

    pub fn generate_f32(value: f32) -> TokenStream {
        let is_negative = value.is_sign_negative();

        if is_negative {
            let value = value.abs();
            quote! { -#value }
        } else {
            quote! { #value }
        }
    }

    pub fn generate_style(style: &Style) -> TokenStream {
        let opacity = Self::generate_f32(style.opacity);
        let transform_translation_x = Self::generate_f32(style.transform_translation_x);

        quote! {
            yoyo::Style {
                opacity: #opacity,
                transform_translation_x: #transform_translation_x,
                ..yoyo::Style::default()
            }
        }
    }

    pub fn generate_transition(transition: &Transition) -> TokenStream {
        match transition {
            Transition::Step => quote! { yoyo::Transition::Step },
            Transition::Delay(delay) => quote! { yoyo::Transition::Delay(#delay) },
            Transition::EaseInOut => quote! { yoyo::Transition::Tween(yoyo::Tween {
                duration: 0.3,
                easing: yoyo::Easing::EaseInOut
            }) },
            Transition::Spring(spring) => {
                let stiffness = spring.stiffness;
                let damping = spring.damping;
                let mass = spring.mass;
                let allows_overdamping = spring.allows_overdamping;
                let overshoot_clamping = spring.overshoot_clamping;

                quote! { yoyo::Transition::Spring(yoyo::Spring {
                    stiffness: #stiffness,
                    damping: #damping,
                    mass: #mass,
                    allows_overdamping: #allows_overdamping,
                    overshoot_clamping: #overshoot_clamping,
                }) }
            }
        }
    }

    pub fn generate_transitions(style: &Style) -> TokenStream {
        let opacity = Self::generate_transition(&style.opacity_transition);
        let transform_translation_x =
            Self::generate_transition(&style.transform_translation_x_transition);

        quote! {
            yoyo::Transitions {
                opacity: #opacity,
                transform_translation_x: #transform_translation_x,
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
                names.insert(from);
            }

            if let Some(to) = edge.to() {
                names.insert(to);
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
