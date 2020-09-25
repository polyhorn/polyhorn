use std::ops::Neg;
use std::str::FromStr;

use crate::stream::{Punct, TokenStream, TokenTree};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unit<T> {
    pub suffix: String,
    pub value: T,
}

impl<T> FromStr for Unit<T>
where
    T: FromStr,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suffix = s.chars().position(|c| c.is_alphabetic()).unwrap_or(s.len());

        match T::from_str(&s[..suffix]) {
            Ok(value) => Ok(Unit {
                suffix: s[suffix..].to_owned(),
                value,
            }),
            Err(_) => Err(()),
        }
    }
}

/// Parses a CSS unit.
pub fn unit_dim<S, T>(tokens: &[TokenTree<S>]) -> (Option<Unit<T>>, &[TokenTree<S>])
where
    S: TokenStream,
    T: FromStr + Neg<Output = T>,
{
    fn unit_dim_impl<S, T>(tokens: &[TokenTree<S>]) -> (Option<Unit<T>>, &[TokenTree<S>])
    where
        S: TokenStream,
        T: FromStr,
    {
        let string = match tokens.first() {
            Some(TokenTree::Literal(literal)) => literal.to_string(),
            _ => return (None, tokens),
        };

        let unit = match Unit::from_str(&string) {
            Ok(unit) => unit,
            Err(_) => return (None, tokens),
        };

        let tokens = &tokens[1..];

        match tokens.first() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '%' && unit.suffix.is_empty() => (
                Some(Unit {
                    suffix: "%".to_string(),
                    value: unit.value,
                }),
                &tokens[1..],
            ),
            _ => (Some(unit), tokens),
        }
    }

    match tokens.first() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
            let (unit, remaining) = unit_dim_impl::<S, T>(&tokens[1..]);

            (
                unit.map(|unit| Unit {
                    suffix: unit.suffix,
                    value: -unit.value,
                }),
                remaining,
            )
        }
        _ => unit_dim_impl(tokens),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Unit;

    #[test]
    fn test_unit_from_str() {
        let unit = Unit::<f32>::from_str("0.25px");
        assert_eq!(
            unit,
            Ok(Unit::<f32> {
                value: 0.25,
                suffix: "px".to_owned()
            })
        )
    }
}
