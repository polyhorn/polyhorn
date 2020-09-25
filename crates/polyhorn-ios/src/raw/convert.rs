use polyhorn_ios_sys::polykit::{
    PLYByEdge, PLYCornerRadii, PLYDimension, PLYDimensionKind, PLYLayoutAxisX, PLYLayoutAxisY,
    PLYPoint,
};
use polyhorn_ios_sys::quartzcore::CATransform3D;
use polyhorn_ios_sys::uikit::{UIColor, UIFont, UIScrollViewIndicatorStyle};
use polyhorn_ui::color::Color;
use polyhorn_ui::font::{Font, FontFamily, FontSize, FontStyle, FontWeight, GenericFontFamily};
use polyhorn_ui::geometry::{ByCorner, ByDirection, ByEdge, Dimension};
use polyhorn_ui::layout::{LayoutAxisX, LayoutAxisY, LayoutDirection};
use polyhorn_ui::linalg::Transform3D;
use polyhorn_ui::styles::ScrollbarColor;

/// Implemented by Polyhorn types that can be trivially converted into native
/// Objective-C types.
pub trait Convert<T> {
    /// This function should perform the conversion to a native Objective-C
    /// type.
    fn convert(self) -> T;
}

impl Convert<UIColor> for Color {
    fn convert(self) -> UIColor {
        let rgb = self.to_srgb();
        UIColor::new(rgb.red as _, rgb.green as _, rgb.blue as _, rgb.alpha as _)
    }
}

impl Convert<PLYPoint> for ByDirection<Dimension<f32>> {
    fn convert(self) -> PLYPoint {
        PLYPoint {
            x: self.horizontal.convert(),
            y: self.vertical.convert(),
        }
    }
}

impl<T1, T2> Convert<PLYLayoutAxisX<T2>> for LayoutAxisX<T1>
where
    T1: Convert<T2>,
{
    fn convert(self) -> PLYLayoutAxisX<T2> {
        match self {
            LayoutAxisX::DirectionDependent { leading, trailing } => PLYLayoutAxisX {
                independent: false,
                start: leading.convert(),
                end: trailing.convert(),
            },
            LayoutAxisX::DirectionIndependent { left, right } => PLYLayoutAxisX {
                independent: true,
                start: left.convert(),
                end: right.convert(),
            },
        }
    }
}

impl<T1, T2> Convert<PLYLayoutAxisY<T2>> for LayoutAxisY<T1>
where
    T1: Convert<T2>,
{
    fn convert(self) -> PLYLayoutAxisY<T2> {
        PLYLayoutAxisY {
            top: self.top.convert(),
            bottom: self.bottom.convert(),
        }
    }
}

impl<T1, T2> Convert<PLYByEdge<T2>> for ByEdge<T1>
where
    T1: Convert<T2>,
{
    fn convert(self) -> PLYByEdge<T2> {
        PLYByEdge {
            horizontal: self.horizontal.convert(),
            vertical: self.vertical.convert(),
        }
    }
}

impl Convert<PLYCornerRadii> for ByCorner<ByDirection<Dimension<f32>>> {
    fn convert(self) -> PLYCornerRadii {
        let direction = LayoutDirection::LTR;

        let top_left = self.top.left(direction).convert();
        let top_right = self.top.right(direction).convert();
        let bottom_right = self.bottom.right(direction).convert();
        let bottom_left = self.bottom.left(direction).convert();

        PLYCornerRadii {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl Convert<PLYDimension> for Dimension<f32> {
    fn convert(self) -> PLYDimension {
        match self {
            Dimension::Points(pixels) => PLYDimension {
                kind: PLYDimensionKind::Pixels,
                value: pixels as _,
            },
            Dimension::Percentage(percent) => PLYDimension {
                kind: PLYDimensionKind::Percentage,
                value: percent as _,
            },
            _ => PLYDimension {
                kind: PLYDimensionKind::Pixels,
                value: 0.0,
            },
        }
    }
}

impl Convert<UIScrollViewIndicatorStyle> for ScrollbarColor {
    fn convert(self) -> UIScrollViewIndicatorStyle {
        match self {
            ScrollbarColor::Auto => UIScrollViewIndicatorStyle::Default,
            ScrollbarColor::Dark => UIScrollViewIndicatorStyle::Black,
            ScrollbarColor::Light => UIScrollViewIndicatorStyle::White,
        }
    }
}

impl Convert<CATransform3D> for Transform3D<f32> {
    fn convert(self) -> CATransform3D {
        // Polyhorn uses `y = Ax` (pre-multiply transform).
        // CoreAnimation uses `y = xA` (post-multiply transform).
        // That's why `m23` refers to the second column and third row of our
        // pre-multiply transformation matrix.
        CATransform3D {
            m11: self.columns[0][0] as _,
            m12: self.columns[0][1] as _,
            m13: self.columns[0][2] as _,
            m14: self.columns[0][3] as _,
            m21: self.columns[1][0] as _,
            m22: self.columns[1][1] as _,
            m23: self.columns[1][2] as _,
            m24: self.columns[1][3] as _,
            m31: self.columns[2][0] as _,
            m32: self.columns[2][1] as _,
            m33: self.columns[2][2] as _,
            m34: self.columns[2][3] as _,
            m41: self.columns[3][0] as _,
            m42: self.columns[3][1] as _,
            m43: self.columns[3][2] as _,
            m44: self.columns[3][3] as _,
        }
    }
}

impl Convert<UIFont> for Font {
    fn convert(self) -> UIFont {
        let size = match self.size {
            FontSize::Medium => 16.0,
            FontSize::Dimension(dimension) => match dimension {
                Dimension::Points(size) => size,
                _ => unimplemented!("Non-points font sizes have not yet been implemented."),
            },
            _ => unimplemented!("Not all named font sizes have yet been implemented."),
        };

        match self.family {
            FontFamily::Generic(generic) => match generic {
                GenericFontFamily::SansSerif => match self.style {
                    FontStyle::Normal => match self.weight {
                        FontWeight::Normal => UIFont::system_font_of_size(size as _),
                        FontWeight::Bold => UIFont::bold_system_font_of_size(size as _),
                        _ => {
                            unimplemented!("Not all named font weights have yet been implemented.")
                        }
                    },
                    _ => unimplemented!("Oblique font styles have not yet been implemented."),
                },
                _ => unimplemented!("Not all generic font families have yet been implemented."),
            },
            _ => unimplemented!("Custom fonts have not yet been implemented."),
        }
    }
}
