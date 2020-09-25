//! Types and traits for working with animations.

use std::future::Future;
use std::time::Duration;

use crate::linalg::Transform3D;

/// Single keyframe within a keyframe animation that describes the value of a
/// property at a particular point in time.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keyframe<T> {
    /// This is the time to which this keyframe applies. The time is relative to
    /// the start of the animation, not to the start of time itself.
    pub time: Duration,

    /// This is the value that the property should have at this point in time.
    pub value: T,
}

/// Collection of keyframes that make up an animation.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeAnimation<T> {
    /// This is the total duration of this animation.
    pub duration: Duration,

    /// These are the keyframes that make up this animation.
    pub keyframes: Vec<Keyframe<T>>,
}

/// Animatable properties.
#[derive(Clone, Debug, PartialEq)]
pub enum Animation {
    /// Animates the opacity property of a view.
    Opacity(KeyframeAnimation<f32>),

    /// Animates the transform property of a view.
    Transform(KeyframeAnimation<Transform3D<f32>>),
}

/// Trait that should be implemented by animation handles that are returned by
/// the animator trait's start function. This handle can be used to wait for an
/// animation to complete (because it is a future) or cancel a pending animation
/// by dropping it.
///
/// Note: the handle itself doesn't necessarily have to implement `Drop`, it
/// might have a field that holds an internal representation that implements
/// `Drop`. For example, on iOS the `AnimationHandle` has a field that holds a
/// reference counted `PLYAnimationHandle` that is written in Objective-C. Once
/// the reference count drops to zero and the Objective-C object is released, it
/// will automatically remove the animation from its view.
pub trait AnimationHandle: Future<Output = ()> + Send + Sync {}

/// This trait should be implemented by types that schedule animations for
/// objects that can be animated.
pub trait Animator {
    /// This is the handle that this animator returns and that can be used to
    /// track pending animations.
    type AnimationHandle: AnimationHandle;

    /// This function should start the given animation and return a handle that
    /// can be used to track and control its progress.
    fn start(&mut self, animation: Animation) -> Self::AnimationHandle;
}

/// This trait is implemented by types that can be animated.
pub trait Animatable {
    /// This is the type that can schedule new animations for this object.
    type Animator: Animator;

    /// This is the command buffer that this object can piggy-back on.
    type CommandBuffer;

    /// This function should invoke the given callback with a mutable reference
    /// to (a new instance of) this object's animator.
    fn animate<F>(&mut self, animations: F)
    where
        F: FnOnce(&mut Self::Animator) + Send + 'static;

    /// This function should invoke the given callback with a mutable reference
    /// to (a new instance of) this object's animator that piggy-backs on the
    /// given command buffer. Explicitly passing a command buffer ensures that
    /// the animations are started in the same UI event loop iteration as the
    /// current render itself.
    fn animate_with_buffer<F>(&mut self, buffer: &mut Self::CommandBuffer, animations: F)
    where
        F: FnOnce(&mut Self::Animator) + Send + 'static;
}
