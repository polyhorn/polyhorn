use polyhorn::prelude::*;
use polyhorn::Reference;
use polyhorn_core::{use_channel, Receiver};
use polyhorn_ui::events::EventListener;
use polyhorn_ui::geometry::{Dimension, Size};
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
        bounds: Reference<Size<f32>>,
        is_present: bool,
    ) -> ViewStyle {
        let bounds = bounds.apply(manager, |&mut bounds| bounds);

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

        let bounds = use_reference!(manager, Default::default());

        let reference = use_reference!(manager, None);

        let state = use_reference!(manager, {
            let variant = match presence {
                Some(presence) if !presence.is_animated() => self.variant,
                _ => T::initial(),
            };

            // TODO: this size is not accurate.
            let size = Size::default();

            State {
                variant,
                opacity: Arc::new(RwLock::new(PropertyState {
                    value: variant.style().opacity,
                    animation: None,
                })),
                transform: Arc::new(RwLock::new(PropertyState {
                    value: Transform::squash(variant.style().transform, size),
                    animation: None,
                })),
            }
        });

        let is_press = use_state!(manager, false);

        let is_present = presence
            .map(|presence| presence.is_present())
            .unwrap_or(true);
        let safe_to_remove = presence.map(|presence| presence.safe_to_remove());

        let variant = if is_present {
            if *is_press.get(manager) {
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

        let marker = use_state!(manager, ()).weak(manager);

        let refresh = use_channel!(manager, move |mut receiver: Receiver<()>| {
            async move {
                while let Some(_) = receiver.next().await {
                    marker.replace(());
                }
            }
        });

        use_layout_effect!(manager, move |link, buffer| {
            state.apply(link, move |state| {
                if state.variant == variant {
                    return;
                }

                reference.apply(link, move |animatable| {
                    if let Some(animatable) = animatable {
                        let handle = TransitionHandle { buffer, animatable };

                        let context = TransitionContext {
                            previous: state.variant,
                            next: variant,
                            is_present,
                            wait_for_removal,
                            refresh,
                        };

                        state.transition(handle, context);
                    }
                });
            });
        });

        let on_pointer_cancel = self.on_pointer_cancel.clone();
        let on_pointer_cancel = manager.bind(move |link, event| {
            is_press.replace(link, false);

            if is_present {
                on_pointer_cancel.emit(event);
            }
        });

        let on_pointer_down = self.on_pointer_down.clone();
        let on_pointer_down = manager.bind(move |link, event| {
            is_press.replace(link, true);

            if is_present {
                on_pointer_down.emit(event);
            }
        });

        let on_pointer_up = self.on_pointer_up.clone();
        let on_pointer_up = manager.bind(move |link, event| {
            is_press.replace(link, false);

            if is_present {
                on_pointer_up.emit(event);
            }
        });

        let opacity = state.get(manager).opacity.read().unwrap().value.to_owned();
        let transform = [
            Transform::with_transform(
                state
                    .get(manager)
                    .transform
                    .read()
                    .unwrap()
                    .value
                    .to_owned(),
            ),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];

        let on_layout = manager.bind(move |link, size| {
            bounds.replace(link, size);
        });

        let style = ViewStyle {
            opacity,
            transform,
            ..self.adjust_style(manager, bounds, is_present)
        };

        let reference = reference.weak(manager);

        poly!(<polyhorn::prelude::View reference=reference style=style
                               on_pointer_cancel={ on_pointer_cancel }
                                 on_pointer_down={ on_pointer_down }
                                     on_layout={ on_layout }
                                   on_pointer_up={ on_pointer_up } ...>
            { manager.children() }
        </polyhorn::prelude::View>)
    }
}
