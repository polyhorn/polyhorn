pub mod algorithm;

pub use algorithm::Algorithm;

use polyhorn_ui::geometry::{Dimension, Point, Size};

pub enum MeasureFunc {
    Boxed(Box<dyn Fn(Size<Dimension<f32>>) -> Size<f32>>),
}

pub struct Layout {
    pub origin: Point<f32>,
    pub size: Size<f32>,
}
