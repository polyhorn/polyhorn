use polyhorn_core::Sender;
use polyhorn_ui::animation::{Animatable, Animation, Animator, Keyframe, KeyframeAnimation};
use polyhorn_ui::geometry::Size;
use polyhorn_ui::linalg::Transform3D;
use polyhorn_ui::styles::Transform;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use yoyo_physics::{Curve, Sampler};

use super::track::{Property, Tracker};
use crate::utils::SharedAnimationHandle;
use crate::Variants;

pub struct TransitionHandle<'a, A>
where
    A: Animatable,
{
    pub buffer: &'a mut A::CommandBuffer,
    pub animatable: &'a mut A,
}

pub struct TransitionContext<A, T>
where
    A: Animatable + 'static,
    T: Variants + Send,
{
    pub previous: T,
    pub next: T,
    pub is_present: bool,
    pub wait_for_removal: Sender<Vec<SharedAnimationHandle<A>>>,
    pub refresh: Sender<()>,
}

pub struct PropertyAnimation<A, T>
where
    A: Animatable,
    T: Property,
    <T::Tracker as Curve>::Velocity: Default,
{
    start: Instant,
    tracker: T::Tracker,
    handle: SharedAnimationHandle<A>,
}

pub struct PropertyState<A, T>
where
    A: Animatable,
    T: Property,
    <T::Tracker as Curve>::Velocity: Default,
{
    pub value: T,
    pub animation: Option<PropertyAnimation<A, T>>,
}

pub struct State<A, T>
where
    A: Animatable,
    T: Variants + Send,
{
    pub variant: T,
    pub opacity: Arc<RwLock<PropertyState<A, f32>>>,
    pub transform: Arc<RwLock<PropertyState<A, Transform3D<f32>>>>,
}

impl<A, T> State<A, T>
where
    A: Animatable,
    T: Variants + Send,
{
    pub fn keyframes<T2>(
        transition: T2::Transition,
        property: &mut PropertyState<A, T2>,
        target: T2,
        wrapper: impl FnOnce(KeyframeAnimation<T2>) -> <A::Animator as Animator>::AnimationHandle,
    ) -> Option<SharedAnimationHandle<A>>
    where
        T2: Property + Send + Sync + std::fmt::Debug + 'static,
        <T2::Tracker as Curve>::Velocity: Default,
        T2::Transition: Default + PartialEq,
    {
        if transition == T2::Transition::default() {
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

                let approximation = animation
                    .tracker
                    .approximate(Instant::now().duration_since(animation.start).as_secs_f32());
                (approximation.value, approximation.velocity)
            }
            None => (property.value, <T2::Tracker as Curve>::Velocity::default()),
        };

        let tracker = T2::track(transition, from, target, velocity);
        property.value = target;

        let sample_rate = 20.0;
        let sampler = Sampler::<_, <T2::Tracker as Tracker>::Threshold>::with_threshold(
            &tracker,
            sample_rate,
            tracker.threshold(),
        );
        let sampler = sampler.keyed().take(2048);
        let keyframes = sampler
            .map(|(time, frame)| Keyframe {
                time: Duration::from_secs_f32(time),
                value: frame.value,
            })
            .collect::<Vec<_>>();

        if keyframes.len() == 2048 {
            panic!(
                "Warning: threshold is never met, animation is manually limited to 2048 samples."
            );
        }

        let duration = keyframes.len() as f32 / sample_rate;

        let handle = SharedAnimationHandle::new(wrapper(KeyframeAnimation {
            duration: Duration::from_secs_f32(duration),
            keyframes,
        }));

        property.animation.replace(PropertyAnimation {
            start: Instant::now(),
            tracker,
            handle: handle.clone(),
        });

        Some(handle)
    }

    pub fn transition_opacity(
        animator: &mut A::Animator,
        opacity: &mut PropertyState<A, f32>,
        context: &mut TransitionContext<A, T>,
    ) -> Option<SharedAnimationHandle<A>> {
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

    pub fn transition_transform(
        animator: &mut A::Animator,
        transform: &mut PropertyState<A, Transform3D<f32>>,
        context: &mut TransitionContext<A, T>,
    ) -> Option<SharedAnimationHandle<A>> {
        // TODO: this size is not accurate.
        let size = Size::default();
        let squash = Transform::squash(context.next.style().transform, size);

        if transform.value == squash {
            return None;
        }

        let transition = T::transitions(&context.previous, &context.next).transform;

        Self::keyframes(transition, transform, squash, |frames| {
            animator.start(Animation::Transform(frames))
        })
    }

    pub fn transition(
        &mut self,
        handle: TransitionHandle<A>,
        mut context: TransitionContext<A, T>,
    ) {
        assert_ne!(Some(context.previous), T::exit());

        self.variant = context.next;

        let opacity = self.opacity.clone();
        let transform = self.transform.clone();

        handle
            .animatable
            .animate_with_buffer(handle.buffer, move |animator| {
                let mut handles = vec![];

                handles.extend(Self::transition_opacity(
                    animator,
                    &mut opacity.write().unwrap(),
                    &mut context,
                ));

                handles.extend(Self::transition_transform(
                    animator,
                    &mut transform.write().unwrap(),
                    &mut context,
                ));

                if !context.is_present {
                    let _ = context.wait_for_removal.try_send(handles);
                }

                let _ = context.refresh.try_send(());
            });
    }
}
