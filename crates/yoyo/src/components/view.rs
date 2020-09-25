use yoyo_physics::{Curve, Sampler};
use num::{Float, NumCast};
use polyhorn::*;
use polyhorn_channel::{use_channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::presence::DynPresence;
use crate::utils::SharedAnimationHandle;
use crate::{Transition, Variants};

pub struct View<T>
where
    T: Variants + Send,
{
    pub presence: Option<Box<dyn DynPresence>>,
    pub variant: T,
    pub style: Style,
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

pub struct PropertyAnimation<T> {
    start: Instant,
    curve: Box<dyn Curve<T> + Send + Sync>,
    handle: SharedAnimationHandle,
}

pub struct PropertyState<T> {
    value: T,
    animation: Option<PropertyAnimation<T>>,
}

pub struct State<T>
where
    T: Variants + Send,
{
    variant: T,
    opacity: Arc<RwLock<PropertyState<f32>>>,
    transform_translation_x: Arc<RwLock<PropertyState<f32>>>,
}

pub struct TransitionHandle<'a> {
    pub buffer: &'a mut CommandBuffer,
    pub view: &'a mut ViewHandle,
}

pub struct TransitionContext<T>
where
    T: Variants + Send,
{
    pub previous: T,
    pub next: T,
    pub is_present: bool,
    pub wait_for_removal: Sender<Vec<SharedAnimationHandle>>,
    pub refresh: Sender<()>,
}

impl<T> State<T>
where
    T: Variants + Send,
{
    pub fn keyframes<T2>(
        transition: Transition,
        property: &mut PropertyState<T2>,
        target: T2,
        wrapper: impl FnOnce(KeyframeAnimation<T2>) -> AnimationHandle,
    ) -> Option<SharedAnimationHandle>
    where
        T2: Float + Send + Sync + std::fmt::Debug + 'static,
    {
        if let Transition::Step = transition {
            property.value = target;

            property.animation.take();

            return None;
        }

        let (from, velocity) = match property.animation.take() {
            Some(animation) => {
                // We drop the animation handle because we are immediately going
                // to schedule a new animation (note that we're on the main
                // thread right now).
                // TODO: the `Drop` implementation of the `AnimationHandle`
                // schedules a new async task with the main Dispatch Queue (since
                // Apple no longer allows us to check directly if we're on the
                // main thread or not), which means the old animation is removed
                // one event loop iteration after scheduling the new one. It
                // would be better if we could cancel the handle directly since
                // we're already sure we're on the main thread.
                let _ = animation.handle;

                let approximation = animation.curve.approximate(
                    <T2 as NumCast>::from(
                        Instant::now().duration_since(animation.start).as_secs_f32(),
                    )
                    .unwrap(),
                );
                (approximation.value, approximation.velocity)
            }
            None => (property.value, T2::zero()),
        };

        let curve = transition.curve(from, target, velocity);
        property.value = target;

        let sampler = Sampler::new(curve.as_ref(), <T2 as NumCast>::from(20.0).unwrap());
        let keyframes = sampler.take(1024).collect::<Vec<_>>();
        let duration = keyframes.len() as f32 / 20.0;

        let handle = SharedAnimationHandle::new(wrapper(KeyframeAnimation {
            duration: Duration::from_secs_f32(duration),
            keyframes: keyframes
                .into_iter()
                .map(|frame| Keyframe {
                    time: Duration::from_secs_f32(frame.time.to_f32().unwrap()),
                    value: frame.value,
                })
                .collect(),
        }));

        property.animation.replace(PropertyAnimation {
            start: Instant::now(),
            curve,
            handle: handle.clone(),
        });

        Some(handle)
    }

    pub fn transition_opacity(
        animator: &mut Animator,
        opacity: &mut PropertyState<f32>,
        context: &mut TransitionContext<T>,
    ) -> Option<SharedAnimationHandle> {
        if opacity.value == context.next.style().opacity {
            return None;
        }

        let transition = T::transitions(&context.previous, &context.next).opacity;

        Self::keyframes(
            transition,
            opacity,
            context.next.style().opacity,
            |frames| animator.start(Animation::Opacity(frames)),
        )
    }

    pub fn transition_transform_translation_x(
        animator: &mut Animator,
        transform: &mut PropertyState<f32>,
        context: &mut TransitionContext<T>,
    ) -> Option<SharedAnimationHandle> {
        if transform.value == context.next.style().transform_translation_x {
            return None;
        }

        let transition = T::transitions(&context.previous, &context.next).transform_translation_x;

        Self::keyframes(
            transition,
            transform,
            context.next.style().transform_translation_x,
            |frames| animator.start(Animation::TransformTranslationX(frames)),
        )
    }

    pub fn transition(&mut self, handle: TransitionHandle, mut context: TransitionContext<T>) {
        assert_ne!(Some(context.previous), T::exit());

        self.variant = context.next;

        let opacity = self.opacity.clone();
        let transform_translation_x = self.transform_translation_x.clone();

        handle
            .view
            .animate_with_buffer(handle.buffer, move |animator| {
                let mut handles = vec![];

                handles.extend(Self::transition_opacity(
                    animator,
                    &mut opacity.write().unwrap(),
                    &mut context,
                ));

                handles.extend(Self::transition_transform_translation_x(
                    animator,
                    &mut transform_translation_x.write().unwrap(),
                    &mut context,
                ));

                if !context.is_present {
                    let _ = context.wait_for_removal.try_send(handles);
                }

                let _ = context.refresh.try_send(());
            });
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

            state.replace(State {
                variant,
                opacity: Arc::new(RwLock::new(PropertyState {
                    value: variant.style().opacity,
                    animation: None,
                })),
                transform_translation_x: Arc::new(RwLock::new(PropertyState {
                    value: variant.style().transform_translation_x,
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
            Vec<SharedAnimationHandle>,
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

                    reference.apply(move |view: &mut ViewHandle| {
                        let handle = TransitionHandle { buffer, view };

                        let context = TransitionContext {
                            previous: state.variant,
                            next: variant,
                            is_present,
                            wait_for_removal,
                            refresh,
                        };

                        state.transition(handle, context);
                    });
                });
            })
        );

        let on_pointer_cancel = self.on_pointer_cancel.clone();
        let on_pointer_cancel = with!((is_press), |event| {
            is_press.replace(false);

            if is_present {
                on_pointer_cancel.call(event);
            }
        });

        let on_pointer_down = self.on_pointer_down.clone();
        let on_pointer_down = with!((is_press), |event| {
            is_press.replace(true);

            if is_present {
                on_pointer_down.call(event);
            }
        });

        let on_pointer_up = self.on_pointer_up.clone();
        let on_pointer_up = with!((is_press), |event| {
            is_press.replace(false);

            if is_present {
                on_pointer_up.call(event);
            }
        });

        let opacity = state
            .apply(|state| state.opacity.read().unwrap().value)
            .unwrap();

        let transform_translation_x = state
            .apply(|state| state.transform_translation_x.read().unwrap().value)
            .unwrap();

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
            Position::Absolute
        };

        let is_absolute = position == Position::Absolute;

        let width = if is_absolute && self.style.width == Dimension::Auto {
            bounds.width.px()
        } else {
            self.style.width
        };

        let height = if is_absolute && self.style.height == Dimension::Auto {
            bounds.height.px()
        } else {
            self.style.height
        };

        poly!(<polyhorn::View reference=reference style={ Style {
            opacity,
            transform_translation_x,
            position,
            width,
            height,
            ..self.style.clone()
        } } on_pointer_cancel={ on_pointer_cancel }
              on_pointer_down={ on_pointer_down }
                on_pointer_up={ on_pointer_up } ...>
            { manager.children() }
        </polyhorn::View>)
    }
}
