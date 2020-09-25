use casco::domain::{Comma, GroupedBy, Parentheses, Parse, SeparatedBy};
use casco::stream::{MultiSpan, Spanned, TokenStream, TokenTree};
use derivative::Derivative;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::color::Color;
use crate::font::{FontFamily, FontSize, FontWeight};
use crate::geometry::{ByCorner, ByDirection, ByEdge, Dimension};
use crate::layout::{LayoutAxisX, LayoutAxisY};
use crate::physics::Angle;
use crate::styles::{Border, Inherited, Transform, TransformBuilder};

/// A non-constructible type that provides a generic namespace for the parser
/// functions.
pub struct Parser<S>(PhantomData<S>);

impl<S> Parse<S> for Dimension<f32>
where
    S: TokenStream,
{
    type Output = Dimension<f32>;

    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
        if let (Some(result), remaining) = Parser::take_enum(tokens) {
            return (Some(result), remaining);
        }

        if let (Some(dim), remaining) = casco::domain::unit_dim(tokens) {
            return (
                Some(match (dim.suffix.as_str(), dim.value) {
                    ("%", value) => Dimension::Percentage(value),
                    ("px", value) => Dimension::Points(value),
                    ("", value) if value == 0.0 => Dimension::Points(0.0),
                    _ => return (None, tokens),
                }),
                remaining,
            );
        }

        (None, tokens)
    }
}

impl<S> Parse<S> for Angle<f32>
where
    S: TokenStream,
{
    type Output = Angle<f32>;

    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
        if let (Some(dim), remaining) = casco::domain::unit_dim(tokens) {
            return (
                Some(match (dim.suffix.as_str(), dim.value) {
                    ("rad", value) => Angle::with_radians(value),
                    ("deg", value) => Angle::with_degrees(value),
                    ("", value) if value == 0.0 => Angle::default(),
                    _ => return (None, tokens),
                }),
                remaining,
            );
        }

        (None, tokens)
    }
}

impl<S> Parser<S>
where
    S: TokenStream,
{
    /// Attempts to consume one or more tokens from the given slice that make up
    /// a name, matches it to the enum `T` using the `FromStr` it implements and
    /// returns both the result and the remaining tokens. If no variant of the
    /// enum matches the name (or if there is no name), this function returns
    /// `None` and the original slice of tokens.
    pub fn take_enum<'a, T>(tokens: &'a [TokenTree<S>]) -> (Option<T>, &'a [TokenTree<S>])
    where
        T: FromStr,
    {
        let (name, remaining) = match casco::domain::name(tokens) {
            (Some(name), remaining) => (name, remaining),
            _ => return (None, tokens),
        };

        match T::from_str(&name) {
            Ok(result) => (Some(result), remaining),
            Err(_) => (None, remaining),
        }
    }

    /// Parses the given slice of tokens to a variant of the given enum. This
    /// function returns an error if the given slice of tokens does not match
    /// any variant of the enum or if tokens remain in the slice even after
    /// parsing the enum variant.
    pub fn parse_enum<T>(tokens: &[TokenTree<S>]) -> Result<T, ParseError<S>>
    where
        T: FromStr,
    {
        let name_span = MultiSpan::new(tokens);
        let (name, remaining) = match casco::domain::name(tokens) {
            (Some(name), remaining) => (name, remaining),
            _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
        };

        if let Some(first) = remaining.first() {
            return Err(ParseError::UnexpectedToken(first.span()));
        }

        match T::from_str(&name) {
            Ok(result) => Ok(result),
            Err(_) => Err(ParseError::UnknownVariant(name_span)),
        }
    }

    /// Parses a color from the given slice of tokens. Returns an error if the
    /// given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a color.
    pub fn parse_color(tokens: &[TokenTree<S>]) -> Result<Color, ParseError<S>> {
        Self::parse_enum(tokens)
    }

    /// Parses an inheritable color from the given slice of tokens. Returns an
    /// error if the given slice is malformed or if tokens remain in the slice
    /// even after successfully parsing a color or the `inherit` word.
    pub fn parse_inherited_color(
        tokens: &[TokenTree<S>],
    ) -> Result<Inherited<Color>, ParseError<S>> {
        match tokens {
            [TokenTree::Ident(ident)] if ident.to_string().as_str() == "inherit" => {
                Ok(Inherited::Inherited)
            }
            _ => Self::parse_color(tokens).map(|color| Inherited::Specified(color)),
        }
    }

    /// Parses a font family from the given slice of tokens. Returns an error if
    /// the given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a font family.
    pub fn parse_font_family(
        tokens: &[TokenTree<S>],
    ) -> Result<Inherited<FontFamily<String>>, ParseError<S>> {
        if let Ok(result) = Self::parse_enum(tokens) {
            return Ok(result);
        }

        if let (Some(name), remaining) = casco::domain::string(tokens) {
            if let Some(remaining) = remaining.first() {
                return Err(ParseError::UnexpectedToken(remaining.span()));
            }

            return Ok(Inherited::Specified(FontFamily::Named(name)));
        }

        Err(ParseError::UnexpectedToken(tokens[0].span()))
    }

    /// Parses a font weight from the given slice of tokens. Returns an error if
    /// the given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a font weight.
    pub fn parse_font_weight(
        tokens: &[TokenTree<S>],
    ) -> Result<Inherited<FontWeight>, ParseError<S>> {
        if let Ok(result) = Self::parse_enum(tokens) {
            return Ok(result);
        }

        Self::parse_number(tokens).map(|number| Inherited::Specified(FontWeight::Number(number)))
    }

    /// Parses a number from the given slice of tokens. Returns an error if the
    /// given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a number.
    pub fn parse_number(tokens: &[TokenTree<S>]) -> Result<f32, ParseError<S>> {
        if let (Some(value), remaining) = casco::domain::number(tokens) {
            if let Some(remaining) = remaining.first() {
                return Err(ParseError::UnexpectedToken(remaining.span()));
            }

            return Ok(value);
        }

        Err(ParseError::UnexpectedToken(tokens[0].span()))
    }

    /// Parses a font size from the given slice of tokens. Returns an error if
    /// the given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a font size.
    pub fn parse_font_size(tokens: &[TokenTree<S>]) -> Result<Inherited<FontSize>, ParseError<S>> {
        if let Ok(result) = Self::parse_enum(tokens) {
            return Ok(result);
        }

        Self::parse_dimension(tokens)
            .map(|dimension| Inherited::Specified(FontSize::Dimension(dimension)))
    }

    /// Attempts to consume one or more tokens from the given slice that make up
    /// a dimension. If no dimension could be consumed, this function returns
    /// `None` and the original slice of tokens.
    pub fn take_dimension<'a>(
        tokens: &'a [TokenTree<S>],
    ) -> (Option<Dimension<f32>>, &'a [TokenTree<S>]) {
        if let (Some(result), remaining) = Self::take_enum(tokens) {
            return (Some(result), remaining);
        }

        if let (Some(dim), remaining) = casco::domain::unit_dim(tokens) {
            return (
                Some(match (dim.suffix.as_str(), dim.value) {
                    ("%", value) => Dimension::Percentage(value),
                    ("px", value) => Dimension::Points(value),
                    ("", value) if value == 0.0 => Dimension::Points(0.0),
                    _ => return (None, tokens),
                }),
                remaining,
            );
        }

        (None, tokens)
    }

    /// Parses a dimension from the given slice of tokens. Returns an error if
    /// the given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a dimension.
    pub fn parse_dimension(tokens: &[TokenTree<S>]) -> Result<Dimension<f32>, ParseError<S>> {
        if let Ok(result) = Self::parse_enum(tokens) {
            return Ok(result);
        }

        if let (Some(dim), remaining) = casco::domain::unit_dim(tokens) {
            if let Some(remaining) = remaining.first() {
                return Err(ParseError::UnexpectedToken(remaining.span()));
            }

            return Ok(match (dim.suffix.as_str(), dim.value) {
                ("%", value) => Dimension::Percentage(value / 100.0),
                ("px", value) => Dimension::Points(value),
                ("", value) if value == 0.0 => Dimension::Points(0.0),
                _ => return Err(ParseError::UnrecognizedUnit(tokens[0].span())),
            });
        }

        Err(ParseError::UnexpectedToken(tokens[0].span()))
    }

    /// Parses a border from the given slice of tokens. Returns an error if the
    /// given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a border.
    pub fn parse_border(tokens: &[TokenTree<S>]) -> Result<ByEdge<Border>, ParseError<S>> {
        let (width, remaining) = match Self::take_dimension(tokens) {
            (Some(width), remaining) => (width, remaining),
            (_, remaining) => return Err(ParseError::UnexpectedToken(remaining[0].span())),
        };

        let (style, remaining) = match Self::take_enum(remaining) {
            (Some(style), remaining) => (style, remaining),
            (_, remaining) => return Err(ParseError::UnexpectedToken(remaining[0].span())),
        };

        let color = Self::parse_color(remaining)?;

        let border = Border {
            width,
            style,
            color,
        };

        Ok(ByEdge {
            horizontal: LayoutAxisX::DirectionIndependent {
                left: border,
                right: border,
            },
            vertical: LayoutAxisY {
                top: border,
                bottom: border,
            },
        })
    }

    /// Parses a border radius from the given slice of tokens. Returns an error
    /// if the given slice is malformed or if tokens remain in the slice even
    /// after successfully parsing a border radius.
    pub fn parse_border_radius(
        tokens: &[TokenTree<S>],
    ) -> Result<ByCorner<ByDirection<Dimension<f32>>>, ParseError<S>> {
        let by_corner = Self::parse_by_corner(tokens, Self::take_dimension)?;

        Ok(by_corner.map(|dim| ByDirection::with_both(dim)))
    }

    /// Parses a generic type for each corner of a rectangle. Returns an error
    /// if the input slice could not be parsed.
    pub fn parse_by_corner<F, T>(
        tokens: &[TokenTree<S>],
        op: F,
    ) -> Result<ByCorner<T>, ParseError<S>>
    where
        for<'a> F: Fn(&'a [TokenTree<S>]) -> (Option<T>, &'a [TokenTree<S>]),
        T: Copy,
    {
        let edges = Self::parse_by_edge(tokens, op)?;

        Ok(ByCorner {
            all: match edges.horizontal {
                LayoutAxisX::DirectionIndependent { left, right } => LayoutAxisY {
                    top: LayoutAxisX::DirectionIndependent {
                        left: edges.vertical.top,
                        right,
                    },
                    bottom: LayoutAxisX::DirectionIndependent {
                        left,
                        right: edges.vertical.bottom,
                    },
                },
                LayoutAxisX::DirectionDependent { leading, trailing } => LayoutAxisY {
                    top: LayoutAxisX::DirectionDependent {
                        leading: edges.vertical.top,
                        trailing,
                    },
                    bottom: LayoutAxisX::DirectionDependent {
                        leading,
                        trailing: edges.vertical.bottom,
                    },
                },
            },
        })
    }

    /// Parses a generic type for each edge of a rectangle. Returns an error if
    /// the input slice could not be parsed.
    pub fn parse_by_edge<F, T>(
        mut tokens: &[TokenTree<S>],
        op: F,
    ) -> Result<ByEdge<T>, ParseError<S>>
    where
        for<'a> F: Fn(&'a [TokenTree<S>]) -> (Option<T>, &'a [TokenTree<S>]),
        T: Copy,
    {
        let span = MultiSpan::new(tokens);
        let mut parts = vec![];

        while !tokens.is_empty() {
            match op(tokens) {
                (Some(part), remaining) => {
                    parts.push(part);
                    tokens = remaining;
                }
                (None, _) => return Err(ParseError::UnexpectedToken(tokens[0].span())),
            }
        }

        if parts.len() == 4 {
            let left = parts.remove(3);
            let bottom = parts.remove(2);
            let right = parts.remove(1);
            let top = parts.remove(0);

            Ok(ByEdge {
                horizontal: LayoutAxisX::independent(left, right),
                vertical: LayoutAxisY { top, bottom },
            })
        } else if parts.len() == 3 {
            Err(ParseError::TooFewArguments(span))
        } else if parts.len() == 2 {
            let horizontal = parts.remove(1);
            let vertical = parts.remove(0);

            Ok(ByEdge {
                horizontal: LayoutAxisX::independent(horizontal, horizontal),
                vertical: LayoutAxisY {
                    top: vertical,
                    bottom: vertical,
                },
            })
        } else if parts.len() == 1 {
            let value = parts.remove(0);

            Ok(ByEdge {
                horizontal: LayoutAxisX::independent(value, value),
                vertical: LayoutAxisY {
                    top: value,
                    bottom: value,
                },
            })
        } else {
            Err(ParseError::TooManyArguments(span))
        }
    }

    /// Parses an angle from the given slice of tokens. Returns an error if the
    /// given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing an angle.
    pub fn parse_angle(tokens: &[TokenTree<S>]) -> Result<Angle<f32>, ParseError<S>> {
        if let (Some(dim), remaining) = casco::domain::unit_dim(tokens) {
            if let Some(remaining) = remaining.first() {
                return Err(ParseError::UnexpectedToken(remaining.span()));
            }

            return Ok(match (dim.suffix.as_str(), dim.value) {
                ("rad", value) => Angle::with_radians(value),
                ("deg", value) => Angle::with_degrees(value),
                ("", value) if value == 0.0 => Angle::default(),
                _ => return Err(ParseError::UnrecognizedUnit(tokens[0].span())),
            });
        }

        Err(ParseError::UnexpectedToken(tokens[0].span()))
    }

    /// Parses a transform from the given slice of tokens. Returns an error if
    /// the given slice is malformed or if tokens remain in the slice even after
    /// successfully parsing a transform.
    pub fn parse_transform(
        mut tokens: &[TokenTree<S>],
    ) -> Result<[Transform<f32>; 8], ParseError<S>> {
        match tokens {
            [TokenTree::Ident(ident)] if ident.to_string() == "none" => {
                return Ok(Default::default())
            }
            _ => {}
        };

        let mut builder = TransformBuilder::new();

        while !tokens.is_empty() {
            let transform = match &tokens[0] {
                TokenTree::Ident(ident) if ident.to_string() == "matrix" => {
                    match GroupedBy::<Parentheses, SeparatedBy<Comma, [f32; 6]>>::parse(
                        &tokens[1..],
                    ) {
                        (Some(entries), _) => Transform::with_matrix(entries),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "matrix3d" => {
                    match GroupedBy::<Parentheses, SeparatedBy<Comma, [f32; 16]>>::parse(
                        &tokens[1..],
                    ) {
                        (Some(entries), _) => Transform::with_matrix3d(entries),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "perspective" => {
                    match GroupedBy::<Parentheses, Dimension<f32>>::parse(&tokens[1..]) {
                        (Some(Dimension::Points(px)), _) => Transform::with_perspective(px),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident)
                    if ident.to_string() == "rotate" || ident.to_string() == "rotateZ" =>
                {
                    match GroupedBy::<Parentheses, Angle<f32>>::parse(&tokens[1..]) {
                        (Some(angle), _) => Transform::with_rotation(0.0, 0.0, 1.0, angle),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "rotate3d" => {
                    match GroupedBy::<Parentheses, SeparatedBy<Comma, (f32, f32, f32, Angle<f32>)>>::parse(&tokens[1..]) {
                        (Some((rx, ry, rz, angle)), _) => Transform::with_rotation(rx, ry, rz, angle),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "rotateX" => {
                    match GroupedBy::<Parentheses, Angle<f32>>::parse(&tokens[1..]) {
                        (Some(angle), _) => Transform::with_rotation(1.0, 0.0, 0.0, angle),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "rotateY" => {
                    match GroupedBy::<Parentheses, Angle<f32>>::parse(&tokens[1..]) {
                        (Some(angle), _) => Transform::with_rotation(0.0, 1.0, 0.0, angle),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "scale" => {
                    match GroupedBy::<Parentheses, SeparatedBy<Comma, (f32, f32)>>::parse(
                        &tokens[1..],
                    ) {
                        (Some((sx, sy)), _) => Transform::with_scale(sx, sy, 1.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "scale3d" => {
                    match GroupedBy::<Parentheses, SeparatedBy<Comma, (f32, f32, f32)>>::parse(
                        &tokens[1..],
                    ) {
                        (Some((sx, sy, sz)), _) => Transform::with_scale(sx, sy, sz),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "scaleX" => {
                    match GroupedBy::<Parentheses, f32>::parse(&tokens[1..]) {
                        (Some(sx), _) => Transform::with_scale(sx, 1.0, 1.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "scaleY" => {
                    match GroupedBy::<Parentheses, f32>::parse(&tokens[1..]) {
                        (Some(sy), _) => Transform::with_scale(1.0, sy, 1.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "scaleZ" => {
                    match GroupedBy::<Parentheses, f32>::parse(&tokens[1..]) {
                        (Some(sz), _) => Transform::with_scale(1.0, 1.0, sz),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "skew" => {
                    return Err(ParseError::Deprecated(
                        MultiSpan::new(&tokens[0..2]),
                        "Skew has been deprecated due to ambiguous behavior.",
                    ));
                }
                TokenTree::Ident(ident) if ident.to_string() == "skewX" => {
                    match GroupedBy::<Parentheses, Angle<f32>>::parse(&tokens[1..]) {
                        (Some(sx), _) => Transform::with_skew_x(sx),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "skewY" => {
                    match GroupedBy::<Parentheses, Angle<f32>>::parse(&tokens[1..]) {
                        (Some(sy), _) => Transform::with_skew_y(sy),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "translate" => {
                    match GroupedBy::<
                        Parentheses,
                        SeparatedBy<Comma, (Dimension<f32>, Dimension<f32>)>,
                    >::parse(&tokens[1..])
                    {
                        (Some((tx, ty)), _) => Transform::with_translation(tx, ty, 0.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "translate3d" => {
                    match GroupedBy::<
                        Parentheses,
                        SeparatedBy<Comma, (Dimension<f32>, Dimension<f32>, f32)>,
                    >::parse(&tokens[1..])
                    {
                        (Some((tx, ty, tz)), _) => Transform::with_translation(tx, ty, tz),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "translateX" => {
                    match GroupedBy::<Parentheses, Dimension<f32>>::parse(&tokens[1..]) {
                        (Some(tx), _) => Transform::with_translation(tx, Dimension::Undefined, 0.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "translateY" => {
                    match GroupedBy::<Parentheses, Dimension<f32>>::parse(&tokens[1..]) {
                        (Some(ty), _) => Transform::with_translation(Dimension::Undefined, ty, 0.0),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                TokenTree::Ident(ident) if ident.to_string() == "translateZ" => {
                    match GroupedBy::<Parentheses, f32>::parse(&tokens[1..]) {
                        (Some(tz), _) => Transform::with_translation(
                            Dimension::Undefined,
                            Dimension::Undefined,
                            tz,
                        ),
                        _ => return Err(ParseError::UnexpectedToken(tokens[0].span())),
                    }
                }
                token => return Err(ParseError::UnexpectedToken(token.span())),
            };

            builder = match builder.push(transform) {
                Ok(builder) => builder,
                _ => unimplemented!("TODO: emit error when there are too many transforms."),
            };
            tokens = &tokens[2..];
        }

        Ok(builder.into_transforms())
    }

    // pub fn parse_function_args
}

/// This is an error that is emitted while parsing the domain-specific property
/// value types.
#[derive(Derivative)]
#[derivative(Copy(bound = ""), Clone(bound = ""), Debug(bound = ""))]
pub enum ParseError<S>
where
    S: TokenStream,
{
    /// This error is emitted when the parser encounters a construct that has
    /// been deprecated.
    Deprecated(MultiSpan<S>, &'static str),

    /// This error is emitted when the parser encounters an unexpected token.
    UnexpectedToken(S::Span),

    /// This error is emitted when parsing an enum if the given variant does not
    /// exist.
    UnknownVariant(MultiSpan<S>),

    /// This error is emitted when the parser encounters a unit it doesn't
    /// recognize. For example, 1vw is currently not yet supported by Polyhorn.
    UnrecognizedUnit(S::Span),

    /// This error is emitted when the parser encounters too few arguments for
    /// multi-part properties. For example, this happens if 3 (instead of 2 or
    /// 4) arguments are provided to margin or padding.
    TooFewArguments(MultiSpan<S>),

    /// This error is emitted when the parser encounters too many arguments for
    /// multi-part properties. For example, this happens if 5 (instead of 2 or
    /// 4) arguments are provided to margin or padding.
    TooManyArguments(MultiSpan<S>),
}
