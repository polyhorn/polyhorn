use super::keywords::Auto;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq)]
pub enum Dimension {
    Undefined,
    Auto,
    Pixels(f32),
    Percent(f32),
}

impl Dimension {
    pub fn auto() -> Dimension {
        Dimension::Auto
    }

    pub fn unwrap_or(self, dimension: Dimension) -> Dimension {
        match self {
            Dimension::Auto | Dimension::Undefined => dimension,
            _ => self,
        }
    }
}

impl Debug for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Dimension::Undefined => f.write_str("undefined"),
            Dimension::Auto => f.write_str("auto"),
            Dimension::Pixels(pixels) => f.write_fmt(format_args!("{}px", pixels)),
            Dimension::Percent(percent) => f.write_fmt(format_args!("{}%", percent * 100.0)),
        }
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl From<Auto> for Dimension {
    fn from(_: Auto) -> Self {
        Dimension::Auto
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Undefined
    }
}

/// This is a trait that is implemented by integer and float types to make it
/// more ergonomic to work with literals (e.g.
/// `5.px() = Dimension::pixels(5.0)`).
pub trait IntoDimension {
    fn px(self) -> Dimension;
    fn pct(self) -> Dimension;
}

impl IntoDimension for isize {
    fn px(self) -> Dimension {
        Dimension::Pixels(self as f32)
    }

    fn pct(self) -> Dimension {
        Dimension::Percent((self as f32) / 100.0)
    }
}

impl IntoDimension for f32 {
    fn px(self) -> Dimension {
        Dimension::Pixels(self)
    }

    fn pct(self) -> Dimension {
        Dimension::Percent(self / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", 2.px()), "2px");
        assert_eq!(format!("{}", 2.5.px()), "2.5px");
    }
}
