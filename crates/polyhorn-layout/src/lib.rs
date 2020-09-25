pub mod algorithm;

pub use algorithm::Algorithm;

mod style;

pub use style::*;

pub enum MeasureFunc {
    Boxed(Box<dyn Fn(Size<polyhorn_style::Dimension>) -> Size<f32>>),
}

pub struct Layout {
    pub origin: Point<f32>,
    pub size: Size<f32>,
}
