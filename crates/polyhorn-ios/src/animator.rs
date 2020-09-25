use super::{Animation, KeyframeAnimation};
use crate::ffi::QueueBound;
use dispatch::Queue;
use futures::channel::oneshot::{channel, Receiver};
use futures::Future;
use polyhorn_ios_sys::{IntoRaw, UIAnimationHandle, UICallback, UIKeyframeAnimation, UIView};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Animator {
    view: UIView,
}

fn convert_keyframes<T>(keyframes: KeyframeAnimation<T>) -> UIKeyframeAnimation
where
    T: Copy + IntoRaw,
{
    UIKeyframeAnimation::new(
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
    pub fn new(view: UIView) -> Animator {
        Animator { view }
    }

    pub fn start(&mut self, animation: Animation) -> AnimationHandle {
        let handle = match animation {
            Animation::Opacity(keyframes) => self
                .view
                .add_animation(convert_keyframes(keyframes), "opacity"),
            Animation::TransformTranslationX(keyframes) => self
                .view
                .add_animation(convert_keyframes(keyframes), "transform.translation.x"),
        };

        AnimationHandle::new(handle)
    }
}

pub struct AnimationHandle {
    #[allow(dead_code)]
    handle: QueueBound<UIAnimationHandle>,
    rx: Receiver<()>,
}

impl AnimationHandle {
    pub fn new(handle: UIAnimationHandle) -> AnimationHandle {
        let mut handle = handle;
        let (tx, rx) = channel();
        let mut tx = Some(tx);

        handle.set_on_stop(UICallback::new(move |_| {
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
