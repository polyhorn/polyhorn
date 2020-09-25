use std::str::FromStr;

use crate::stream::{TokenStream, TokenTree};

/// Parses a CSS number literal.
pub fn number<S>(tokens: &[TokenTree<S>]) -> (Option<f32>, &[TokenTree<S>])
where
    S: TokenStream,
{
    let string = match tokens.first() {
        Some(TokenTree::Literal(literal)) => literal.to_string(),
        _ => return (None, tokens),
    };

    match f32::from_str(&string) {
        Ok(value) => (Some(value), &tokens[1..]),
        Err(_) => (None, tokens),
    }
}

fn unescape(input: &str) -> String {
    let mut chars = input.chars();
    let mut results = vec![];

    while let Some(value) = chars.next() {
        match value {
            '\\' => {
                if let Some(value) = chars.next() {
                    results.push(value)
                }
            }
            value => results.push(value),
        }
    }

    results.into_iter().collect()
}

/// Parses a CSS string literal.
pub fn string<S>(tokens: &[TokenTree<S>]) -> (Option<String>, &[TokenTree<S>])
where
    S: TokenStream,
{
    // Try to parse the first literal.
    let result = match tokens.first() {
        Some(TokenTree::Literal(literal)) => literal.to_string(),
        _ => return (None, tokens),
    };

    match result.chars().next() {
        Some(value) if value == '"' => {}
        _ => return (None, tokens),
    };

    let unescaped = unescape(&result[1..result.len() - 1]);
    (Some(unescaped), &tokens[1..])
}

#[cfg(test)]
mod tests {
    use super::unescape;

    #[test]
    fn test_unescape() {
        assert_eq!(unescape("hello\\\"world"), "hello\"world");
        assert_eq!(unescape("hello\\\\\"world"), "hello\\\"world");
    }
}
