use yoyo_physics as physics;
use num::{Float, NumCast};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Easing {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bezier([f32; 4]),
}

impl Easing {
    pub fn control_points(self) -> [f32; 4] {
        match self {
            Easing::Linear => [0.0, 0.0, 1.0, 1.0],
            Easing::Ease => [0.25, 0.1, 0.25, 1.0],
            Easing::EaseIn => [0.42, 0.0, 1.0, 1.0],
            Easing::EaseOut => [0.0, 0.0, 0.58, 1.0],
            Easing::EaseInOut => [0.42, 0.0, 0.58, 1.0],
            Easing::Bezier(points) => points,
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::Linear
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tween {
    pub duration: f32,
    pub easing: Easing,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub allows_overdamping: bool,
    pub overshoot_clamping: bool,
}

impl Default for Spring {
    fn default() -> Self {
        Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            allows_overdamping: false,
            overshoot_clamping: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Transition {
    Step,
    Delay(f32),
    Tween(Tween),
    Spring(Spring),
}

impl Default for Transition {
    fn default() -> Self {
        Transition::Step
    }
}

impl Transition {
    pub fn curve<T>(
        self,
        from_value: T,
        to_value: T,
        velocity: T,
    ) -> Box<dyn physics::Curve<T> + Send + Sync>
    where
        T: Float + Send + Sync + 'static,
    {
        match self {
            Transition::Step => {
                unimplemented!("Step transitions are (ironically) not yet implemented.")
            }
            Transition::Tween(tween) => {
                let control_points = tween.easing.control_points();

                let curve = physics::bezier::Bezier {
                    from_value,
                    to_value,
                    duration: <T as NumCast>::from(tween.duration).unwrap(),
                    control_points: [
                        (
                            <T as NumCast>::from(control_points[0]).unwrap(),
                            <T as NumCast>::from(control_points[1]).unwrap(),
                        ),
                        (
                            <T as NumCast>::from(control_points[2]).unwrap(),
                            <T as NumCast>::from(control_points[3]).unwrap(),
                        ),
                    ],
                };

                Box::new(curve)
            }
            Transition::Spring(spring) => {
                let curve = physics::spring::Spring {
                    from_value,
                    to_value,
                    initial_velocity: velocity,
                    stiffness: <T as NumCast>::from(spring.stiffness).unwrap(),
                    damping: <T as NumCast>::from(spring.damping).unwrap(),
                    mass: <T as NumCast>::from(spring.mass).unwrap(),
                    allows_overdamping: spring.allows_overdamping,
                    overshoot_clamping: spring.overshoot_clamping,
                };

                Box::new(curve)
            }
            Transition::Delay(delay) => Box::new(physics::delay::Delay {
                from_value,
                to_value,
                duration: <T as NumCast>::from(delay).unwrap(),
            }),
        }
    }
}
