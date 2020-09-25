use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keyframe<T> {
    pub time: Duration,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeAnimation<T> {
    pub duration: Duration,
    pub keyframes: Vec<Keyframe<T>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Animation {
    Opacity(KeyframeAnimation<f32>),
    TransformTranslationX(KeyframeAnimation<f32>),
}
