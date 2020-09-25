use polyhorn::prelude::*;
use polyhorn::Reference;
use polyhorn_channel::{use_channel, Receiver};
use polyhorn_ui::events::EventListener;
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::handles::{Imperative, ViewHandle};
use polyhorn_ui::styles::{Position, Transform, ViewStyle};
use std::sync::{Arc, RwLock};

use super::{PropertyState, State, TransitionContext, TransitionHandle};
use crate::components::presence::DynPresence;
use crate::utils::SharedAnimationHandle;
use crate::Variants;

pub struct View<T>
where
    T: Variants + Send,
{
    pub presence: Option<Box<dyn DynPresence>>,
    pub variant: T,
    pub style: ViewStyle,
    pub on_pointer_cancel: EventListener<()>,
    pub on_pointer_down: EventListener<()>,
    pub on_pointer_up: EventListener<()>,
}

impl<T> Default for View<T>
where
    T: Variants + Send,
{
    fn default() -> Self {
        View {
            presence: None,
            variant: T::initial(),
            style: Default::default(),
            on_pointer_cancel: Default::default(),
            on_pointer_down: Default::default(),
            on_pointer_up: Default::default(),
        }
    }
}

impl<T> View<T>
where
    T: Variants + Send,
{
    pub fn adjust_style(
        &self,
        manager: &mut Manager,
        is_present: bool,
        reference: &Reference<<polyhorn::prelude::View as Imperative>::Handle>,
    ) -> ViewStyle {
        let bounds = use_reference!(manager);

        let bounds_sender = use_channel!(
            manager,
            with!((bounds), |mut receiver| {
                async move {
                    while let Some(value) = receiver.next().await {
                        bounds.replace(value);
                    }
                }
            })
        );

        if is_present {
            use_effect!(
                manager,
                with!((reference, bounds_sender), |buffer| {
                    let mut reference = reference;
                    let mut bounds_sender = bounds_sender;

                    reference.apply(|view| {
                        view.size_with_buffer(buffer, move |bounds| {
                            let _ = bounds_sender.try_send(bounds);
                        })
                    });
                })
            );
        }

        let bounds = bounds.to_owned().unwrap_or_default();

        let position = if is_present {
            self.style.position
        } else {
            Position::Absolute(Default::default())
        };

        let is_absolute = matches!(position, Position::Absolute(_));

        let width = if is_absolute && self.style.size.width == Dimension::Auto {
            Dimension::Points(bounds.width)
        } else {
            self.style.size.width
        };

        let height = if is_absolute && self.style.size.height == Dimension::Auto {
            Dimension::Points(bounds.height)
        } else {
            self.style.size.height
        };

        ViewStyle {
            position,
            size: Size::new(width, height),
            ..self.style.clone()
        }
    }
}

impl<T> Component for View<T>
where
    T: Variants + Send,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let presence = self.presence.as_ref();

        let reference = use_reference!(manager);
        let mut state = use_reference!(manager);
        let marker = use_state!(manager, ());

        let is_press = use_state!(manager, false);

        let is_present = presence
            .map(|presence| presence.is_present())
            .unwrap_or(true);
        let safe_to_remove = presence.map(|presence| presence.safe_to_remove());

        if state.is_none() {
            let variant = match presence {
                Some(presence) if !presence.is_animated() => self.variant,
                _ => T::initial(),
            };

            // TODO: this size is not accurate.
            let size = Size::default();

            state.replace(State {
                variant,
                opacity: Arc::new(RwLock::new(PropertyState {
                    value: variant.style().opacity,
                    animation: None,
                })),
                transform: Arc::new(RwLock::new(PropertyState {
                    value: Transform::squash(variant.style().transform, size),
                    animation: None,
                })),
            });
        }

        let variant = if is_present {
            if is_press.to_owned() {
                T::press().unwrap_or(self.variant)
            } else {
                self.variant
            }
        } else if let Some(variant) = T::exit() {
            variant
        } else {
            if let Some(safe_to_remove) = safe_to_remove.as_ref() {
                safe_to_remove.invoke();
            }

            self.variant
        };

        let wait_for_removal = use_channel!(manager, move |mut receiver: Receiver<
            Vec<SharedAnimationHandle<_>>,
        >| {
            async move {
                if let Some(animations) = receiver.next().await {
                    for animation in animations {
                        if let Some(animation) = animation.take() {
                            animation.await
                        }
                    }

                    if let Some(safe_to_remove) = safe_to_remove {
                        safe_to_remove.invoke();
                    }
                }
            }
        });

        let refresh = use_channel!(
            manager,
            with!((marker), |mut receiver: Receiver<()>| {
                async move {
                    while let Some(_) = receiver.next().await {
                        marker.replace(());
                    }
                }
            })
        );

        use_effect!(
            manager,
            with!((reference, state, marker), |buffer| {
                let mut reference = reference;
                let mut state = state;

                state.apply(move |state| {
                    if state.variant == variant {
                        return;
                    }

                    reference.apply(
                        move |animatable: &mut <polyhorn::prelude::View as Imperative>::Handle| {
                            let handle = TransitionHandle { buffer, animatable };

                            let context = TransitionContext {
                                previous: state.variant,
                                next: variant,
                                is_present,
                                wait_for_removal,
                                refresh,
                            };

                            state.transition(handle, context);
                        },
                    );
                });
            })
        );

        let on_pointer_cancel = self.on_pointer_cancel.clone();
        let on_pointer_cancel = with!((is_press), |event| {
            is_press.replace(false);

            if is_present {
                on_pointer_cancel.emit(event);
            }
        });

        let on_pointer_down = self.on_pointer_down.clone();
        let on_pointer_down = with!((is_press), |event| {
            is_press.replace(true);

            if is_present {
                on_pointer_down.emit(event);
            }
        });

        let on_pointer_up = self.on_pointer_up.clone();
        let on_pointer_up = with!((is_press), |event| {
            is_press.replace(false);

            if is_present {
                on_pointer_up.emit(event);
            }
        });

        let style = ViewStyle {
            opacity: state
                .apply(|state| state.opacity.read().unwrap().value)
                .unwrap(),
            transform: [
                Transform::with_transform(
                    state
                        .apply(|state| state.transform.read().unwrap().value)
                        .unwrap(),
                ),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            ..self.adjust_style(manager, is_present, &reference)
        };

        poly!(<polyhorn::prelude::View reference=reference style=style
                               on_pointer_cancel={ on_pointer_cancel }
                                 on_pointer_down={ on_pointer_down }
                                   on_pointer_up={ on_pointer_up } ...>
            { manager.children() }
        </polyhorn::prelude::View>)
    }
}
