//! Custom encodings for alternative color spaces.

use palette::encoding::Srgb;
use palette::float::Float;
use palette::rgb::{Primaries, RgbSpace, RgbStandard};
use palette::white_point::{WhitePoint, D65};
use palette::{Component, Yxy};

/// This is the Display-P3 color space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisplayP3;

impl RgbStandard for DisplayP3 {
    type Space = DisplayP3;

    /// DisplayP3 uses the same transfer function as sRGB.
    type TransferFn = Srgb;
}

impl RgbSpace for DisplayP3 {
    type Primaries = DisplayP3;
    type WhitePoint = D65;
}

impl Primaries for DisplayP3 {
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        // NOTE: the Y coordinate is actually 0.00, but this results in a
        // non-invertible transformation matrix.
        Yxy::with_wp(0.680.convert(), 0.320.convert(), 0.001.convert())
    }

    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(0.265.convert(), 0.690.convert(), 0.045.convert())
    }

    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T> {
        Yxy::with_wp(0.150.convert(), 0.060.convert(), 0.79.convert())
    }
}
