use casco::stream::{MultiSpan, TokenStream};
use derivative::Derivative;

use super::{Property, PropertyValue, Style, TransformTransition, Transitions};

/// This is a diagnostic that is emitted while building a style.
#[derive(Derivative)]
#[derivative(Debug, PartialEq(bound = "S::Span: PartialEq"))]
pub enum Diagnostic<S>
where
    S: TokenStream,
{
    /// This diagnostic is emitted when a property is overridden by another
    /// property.
    UnusedProperty(MultiSpan<S>),
}

/// This is a structure that is used to build the style from a series of
/// properties.
pub struct StyleBuilder<S>
where
    S: TokenStream,
{
    diagnostics: Vec<Diagnostic<S>>,
}

struct Tracked<T, S>
where
    S: TokenStream,
{
    value: T,
    previous_span: Option<MultiSpan<S>>,
}

impl<S> StyleBuilder<S>
where
    S: TokenStream,
{
    /// Returns a new style builder.
    pub fn new() -> StyleBuilder<S> {
        StyleBuilder {
            diagnostics: vec![],
        }
    }

    fn track<T>(&mut self, value: T) -> Tracked<T, S> {
        Tracked {
            value,
            previous_span: None,
        }
    }

    fn replace<T>(&mut self, tracker: &mut Tracked<T, S>, value: T, span: MultiSpan<S>) {
        if let Some(span) = tracker.previous_span {
            self.diagnostics.push(Diagnostic::UnusedProperty(span));
        }

        tracker.value = value;
        tracker.previous_span = Some(span);
    }

    pub fn build_transitions(
        &mut self,
        properties: &[Property<S>],
        default: Transitions,
    ) -> Transitions {
        let mut opacity = self.track(default.opacity);
        let mut transform = self.track(default.transform);

        for property in properties {
            match &property.value {
                PropertyValue::TransitionOpacity(value) => {
                    self.replace(&mut opacity, *value, property.value_span);
                }
                PropertyValue::TransitionTransform(value) => {
                    self.replace(
                        &mut transform,
                        TransformTransition {
                            translation: *value,
                            scale: *value,
                            skew: *value,
                            perspective: *value,
                            rotation: *value,
                        },
                        property.value_span,
                    );
                }
                _ => {}
            }
        }

        Transitions {
            opacity: opacity.value,
            transform: transform.value,
        }
    }

    pub fn build_style(&mut self, properties: &[Property<S>], default: Style) -> Style {
        let mut opacity = self.track(default.opacity);
        let mut transform = self.track(default.transform);

        for property in properties {
            match &property.value {
                PropertyValue::Opacity(value) => {
                    self.replace(&mut opacity, *value, property.value_span);
                }
                PropertyValue::Transform(value) => {
                    self.replace(&mut transform, *value, property.value_span);
                }
                _ => {}
            }
        }

        Style {
            opacity: opacity.value,
            transform: transform.value,
            transitions: self.build_transitions(properties, default.transitions),
        }
    }

    // Currently we don't yet emit diagnostics because there are some subtle
    // challenges caused by the fact that a variant can be affected by different
    // declarations.
    #[allow(dead_code)]
    pub fn into_diagnostics(self) -> Vec<Diagnostic<S>> {
        self.diagnostics
    }
}
