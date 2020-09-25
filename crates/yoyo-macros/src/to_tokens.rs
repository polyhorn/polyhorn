use casco::stream::MultiSpan;
use polyhorn_ui::macros::style::{CompileError, ParseError};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{Driver, Error, TransformTransition, Transition};

impl ToTokens for Transition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Transition::Step => quote! { yoyo::Transition::Step },
            Transition::Delay(delay) => quote! { yoyo::Transition::Delay(#delay) },
            Transition::EaseInOut(duration) => quote! { yoyo::Transition::Tween(yoyo::Tween {
                duration: #duration,
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
        })
    }
}

impl ToTokens for TransformTransition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let translation = &self.translation;
        let scale = &self.scale;
        let skew = &self.skew;
        let perspective = &self.perspective;
        let rotation = &self.rotation;

        tokens.extend(quote! {
            yoyo::TransformTransition {
                translation: #translation,
                scale: #scale,
                skew: #skew,
                perspective: #perspective,
                rotation: #rotation,
            }
        })
    }
}

pub struct CascoError(
    pub casco::Error<Driver<casco::proc_macro2::TokenStream>, casco::proc_macro2::TokenStream>,
);

impl ToTokens for CascoError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            casco::Error::Cascade(error) => match error {
                &casco::cascade::Error::MissingColon(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing colon.").to_tokens(tokens)
                }
                &casco::cascade::Error::MissingPropertyName(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing property name.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingPropertyValue(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing property value.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingRuleGroup(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing rule group.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingSelector(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing selector.")
                        .to_tokens(tokens)
                }
            },
            casco::Error::Domain(error) => match error {
                &Error::UnexpectedToken(span) => {
                    CompileError::new(MultiSpan::single(span), "Unexpected token.")
                        .to_tokens(tokens)
                }
                &Error::UnrecognizedProperty(span) => {
                    CompileError::new(span, "Unrecognized property.").to_tokens(tokens)
                }
                Error::Parse(error) => match error {
                    &ParseError::Deprecated(span, message) => {
                        CompileError::new(span, message).to_tokens(tokens)
                    }
                    &ParseError::TooFewArguments(span) => {
                        CompileError::new(span, "Too few arguments.").to_tokens(tokens)
                    }
                    &ParseError::TooManyArguments(span) => {
                        CompileError::new(span, "Too many arguments.").to_tokens(tokens)
                    }
                    &ParseError::UnexpectedToken(span) => {
                        CompileError::new(MultiSpan::single(span), "Unexpected token.")
                            .to_tokens(tokens)
                    }
                    &ParseError::UnknownVariant(span) => {
                        CompileError::new(span, "Unknown variant.").to_tokens(tokens)
                    }
                    &ParseError::UnrecognizedUnit(span) => CompileError::new(
                        MultiSpan::single(span),
                        "Unrecognized CSS unit. Only `px` and `%` are supported at this moment.",
                    )
                    .to_tokens(tokens),
                },
            },
        }
    }
}
