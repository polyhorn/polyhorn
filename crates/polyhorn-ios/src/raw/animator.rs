use dispatch::Queue;
use futures::channel::oneshot::{channel, Receiver};
use futures::Future;
use polyhorn_ios_sys::polykit::{PLYAnimationHandle, PLYCallback, PLYKeyframeAnimation, PLYView};
use polyhorn_ios_sys::quartzcore::CATransform3D;
use polyhorn_ios_sys::IntoRaw;
use polyhorn_ui::animation::{Animation, Keyframe, KeyframeAnimation};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::{Convert, QueueBound};

/// Concrete implementation of an animator that animates `UIView`s and related
/// classes.
pub struct Animator {
    view: PLYView,
}

fn convert_keyframes<T>(keyframes: KeyframeAnimation<T>) -> PLYKeyframeAnimation
where
    T: Copy + IntoRaw,
{
    PLYKeyframeAnimation::new(
        keyframes.duration.as_secs_f64(),
        keyframes
            .keyframes
            .iter()
            .map(|frame| frame.time.as_secs_f64() / keyframes.duration.as_secs_f64())
            .collect::<Vec<_>>()
            .as_slice(),
        keyframes
            .keyframes
            .iter()
            .map(|frame| frame.value)
            .collect::<Vec<_>>()
            .as_slice(),
    )
}

impl Animator {
    /// Returns a new animator for the given view.
    pub fn new(view: PLYView) -> Animator {
        Animator { view }
    }
}

impl polyhorn_ui::animation::Animator for Animator {
    type AnimationHandle = AnimationHandle;

    fn start(&mut self, animation: Animation) -> AnimationHandle {
        let handle = match animation {
            Animation::Opacity(keyframes) => self
                .view
                .add_animation(convert_keyframes(keyframes), "opacity"),
            Animation::Transform(keyframes) => self.view.add_animation(
                convert_keyframes(KeyframeAnimation {
                    keyframes: keyframes
                        .keyframes
                        .into_iter()
                        .map(|keyframe| Keyframe {
                            time: keyframe.time,
                            value: Convert::<CATransform3D>::convert(keyframe.value),
                        })
                        .collect(),
                    duration: keyframes.duration,
                }),
                "transform",
            ),
        };

        AnimationHandle::new(handle)
    }
}

/// Concrete implementation of a handle that controls the animation of a
/// `UIView` or a related class.
pub struct AnimationHandle {
    #[allow(dead_code)]
    handle: QueueBound<PLYAnimationHandle>,
    rx: Receiver<()>,
}

impl AnimationHandle {
    /// Returns a new wrapper around the given Objective-C animation handle.
    pub fn new(handle: PLYAnimationHandle) -> AnimationHandle {
        let mut handle = handle;
        let (tx, rx) = channel();
        let mut tx = Some(tx);

        handle.set_on_stop(PLYCallback::new(move |_| {
            if let Some(tx) = tx.take() {
                let _ = tx.send(());
            }
        }));

        let handle = unsafe { QueueBound::adopt(Queue::main(), handle) };

        AnimationHandle { handle, rx }
    }
}

impl Future for AnimationHandle {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.rx).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(()),
        }
    }
}

impl polyhorn_ui::animation::AnimationHandle for AnimationHandle {}
