use num_traits::{Float, NumCast};
use polyhorn_ui::linalg::inter::Decomposition3D;
use polyhorn_ui::linalg::{Quaternion3D, Transform3D};
use yoyo_physics::threshold::{And, DisplacementThreshold, VelocityThreshold};
use yoyo_physics::{Approximation, Curve, Threshold};

use crate::{TransformTransition, Transition};

/// This trait is implemented by property values that can be tracked.
pub trait Property: Copy + Default + Sized
where
    <Self::Tracker as Curve>::Velocity: Default,
{
    type Tracker: Tracker<Value = Self>;
    type Transition;

    fn track(
        transition: Self::Transition,
        from: Self,
        to: Self,
        velocity: <Self::Tracker as Curve>::Velocity,
    ) -> Self::Tracker;
}

/// This trait is implemented by property value trackers.
pub trait Tracker: Curve + Sized
where
    Self::Value: Property<Tracker = Self>,
    Self::Velocity: Default,
{
    type Threshold: Threshold<Value = Self::Value, Velocity = Self::Velocity>;

    fn threshold(&self) -> Self::Threshold;
}

/// This is a concrete implementation of the tracker trait for scalar primitives
/// such as f32.
pub struct ScalarTracker<T>(Box<dyn Curve<Value = T, Velocity = T> + Send + Sync>);

impl<T> Tracker for ScalarTracker<T>
where
    T: Property<Tracker = Self> + Float,
{
    type Threshold = And<VelocityThreshold<T>, DisplacementThreshold<T>>;

    fn threshold(&self) -> Self::Threshold {
        And(
            VelocityThreshold(<T as NumCast>::from(0.001).unwrap()),
            DisplacementThreshold {
                target: self.target(),
                sensitivity: <T as NumCast>::from(0.001).unwrap(),
            },
        )
    }
}

impl<T> Curve for ScalarTracker<T>
where
    T: Property<Tracker = Self> + Float,
{
    type Value = T;
    type Velocity = T;

    fn approximate(&self, time: f32) -> Approximation<Self::Value, Self::Velocity> {
        self.0.approximate(time)
    }

    fn target(&self) -> Self::Value {
        self.0.target()
    }
}

impl Property for f32 {
    type Tracker = ScalarTracker<f32>;
    type Transition = Transition;

    fn track(
        transition: Self::Transition,
        from: Self,
        to: Self,
        velocity: <Self::Tracker as Curve>::Velocity,
    ) -> Self::Tracker {
        ScalarTracker(transition.curve(from, to, velocity))
    }
}

pub struct TransformTracker<T>
where
    T: Property,
    T::Tracker: Curve<Velocity = T>,
{
    target: Transform3D<T>,
    composition: Decomposition3D<T::Tracker>,
    quaternions: (Quaternion3D<T>, Quaternion3D<T>),
    quaternion_tracker: ScalarTracker<T>,
}

impl<T> Curve for TransformTracker<T>
where
    T: Property<Transition = Transition, Tracker = ScalarTracker<T>> + Float,
{
    type Value = Transform3D<T>;
    type Velocity = (Decomposition3D<T>, T);

    fn approximate(&self, time: f32) -> Approximation<Self::Value, Self::Velocity> {
        // We start by linearly interpolating the elements of the decomposition.
        let mut approx = self
            .composition
            .as_ref()
            .map(|element| element.approximate(time));

        // Then, we interpolate the quaternions.
        let progress = self.quaternion_tracker.approximate(time);
        let (from, to) = self.quaternions;
        let mix = from.mix(T::one() - progress.value, to);

        for i in 0..4 {
            // Important: we take the value of the quaternion, but we use the
            // velocity of the underlying weight!
            approx.quaternion[i] = Approximation {
                value: mix[i],
                velocity: progress.velocity,
            };
        }

        Approximation {
            value: approx.map(|Approximation { value, .. }| value).recompose(),
            velocity: (
                approx.map(|Approximation { velocity, .. }| velocity),
                progress.velocity,
            ),
        }
    }

    fn target(&self) -> Self::Value {
        self.target
    }
}

pub struct TransformThreshold<T, V>
where
    T: Threshold,
    V: Threshold,
{
    value: Transform3D<T>,
    velocity: V,
}

impl<T, V> Threshold for TransformThreshold<T, V>
where
    T: Threshold,
    V: Threshold,
    T::Value: Copy,
    T::Velocity: Default,
    V::Value: Default,
    V::Velocity: Copy,
{
    type Value = Transform3D<T::Value>;
    type Velocity = (Decomposition3D<V::Velocity>, V::Velocity);

    fn evaluate(&mut self, approximation: &Approximation<Self::Value, Self::Velocity>) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !self.value.columns[i][j].evaluate(&Approximation {
                    value: approximation.value.columns[i][j],
                    velocity: T::Velocity::default(),
                }) {
                    return false;
                }
            }
        }

        for i in 0..3 {
            if !self.velocity.evaluate(&Approximation {
                value: V::Value::default(),
                velocity: approximation.velocity.0.translation[i],
            }) {
                return false;
            }

            if !self.velocity.evaluate(&Approximation {
                value: V::Value::default(),
                velocity: approximation.velocity.0.scale[i],
            }) {
                return false;
            }

            if !self.velocity.evaluate(&Approximation {
                value: V::Value::default(),
                velocity: approximation.velocity.0.skew[i],
            }) {
                return false;
            }
        }

        for i in 0..4 {
            if !self.velocity.evaluate(&Approximation {
                value: V::Value::default(),
                velocity: approximation.velocity.0.perspective[i],
            }) {
                return false;
            }
        }

        self.velocity.evaluate(&Approximation {
            value: V::Value::default(),
            velocity: approximation.velocity.1,
        })
    }
}

impl<T> Tracker for TransformTracker<T>
where
    T: Property<Transition = Transition, Tracker = ScalarTracker<T>> + Float,
{
    type Threshold = TransformThreshold<DisplacementThreshold<T>, VelocityThreshold<T>>;

    fn threshold(&self) -> Self::Threshold {
        TransformThreshold {
            value: self.target.map(|target| DisplacementThreshold {
                target,
                sensitivity: T::from(0.001).unwrap(),
            }),
            velocity: VelocityThreshold(T::from(0.001).unwrap()),
        }
    }
}

impl<T> Property for Transform3D<T>
where
    T: Property<Transition = Transition, Tracker = ScalarTracker<T>> + Float,
{
    type Tracker = TransformTracker<T>;
    type Transition = TransformTransition;

    fn track(
        transition: Self::Transition,
        from: Self,
        to: Self,
        velocity: <Self::Tracker as Curve>::Velocity,
    ) -> Self::Tracker {
        let target = to;

        let from = Decomposition3D::decompose(from).unwrap();
        let to = Decomposition3D::decompose(to).unwrap();

        let (tx, ty, tz) = {
            let transition = transition.translation;
            let from = from.translation;
            let to = to.translation;
            let velocity = velocity.0.translation;

            let x = T::track(transition, from[0], to[0], velocity[0]);
            let y = T::track(transition, from[1], to[1], velocity[1]);
            let z = T::track(transition, from[2], to[2], velocity[2]);

            (x, y, z)
        };

        let (sx, sy, sz) = {
            let transition = transition.scale;
            let from = from.scale;
            let to = to.scale;
            let velocity = velocity.0.scale;

            let x = T::track(transition, from[0], to[0], velocity[0]);
            let y = T::track(transition, from[1], to[1], velocity[1]);
            let z = T::track(transition, from[2], to[2], velocity[2]);

            (x, y, z)
        };

        let (skx, sky, skz) = {
            let transition = transition.skew;
            let from = from.skew;
            let to = to.skew;
            let velocity = velocity.0.skew;

            let x = T::track(transition, from[0], to[0], velocity[0]);
            let y = T::track(transition, from[1], to[1], velocity[1]);
            let z = T::track(transition, from[2], to[2], velocity[2]);

            (x, y, z)
        };

        let (px, py, pz, pw) = {
            let transition = transition.perspective;
            let from = from.perspective;
            let to = to.perspective;
            let velocity = velocity.0.perspective;

            let x = T::track(transition, from[0], to[0], velocity[0]);
            let y = T::track(transition, from[1], to[1], velocity[1]);
            let z = T::track(transition, from[2], to[2], velocity[2]);
            let w = T::track(transition, from[3], to[3], velocity[3]);

            (x, y, z, w)
        };

        // TODO: these are never actually used, so it would be worthwhile to
        // figure out a way to replace them with voids.
        let (rx, ry, rz, rw) = {
            let transition = transition.rotation;
            let from = from.quaternion;
            let to = to.quaternion;
            let velocity = velocity.0.quaternion;

            let x = T::track(transition, from[0], to[0], velocity[0]);
            let y = T::track(transition, from[1], to[1], velocity[1]);
            let z = T::track(transition, from[2], to[2], velocity[2]);
            let w = T::track(transition, from[3], to[3], velocity[3]);

            (x, y, z, w)
        };

        TransformTracker {
            target,
            composition: Decomposition3D {
                translation: [tx, ty, tz],
                scale: [sx, sy, sz],
                skew: [skx, sky, skz],
                perspective: [px, py, pz, pw],
                quaternion: Quaternion3D::new(rx, ry, rz, rw),
            },
            quaternions: (from.quaternion, to.quaternion),
            quaternion_tracker: T::track(transition.rotation, T::zero(), T::one(), velocity.1),
        }
    }
}
