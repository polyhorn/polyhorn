use crate::stream::{Punct, TokenStream, TokenTree};

/// Parses a CSS name (a series of `Ident`s separated by `-` `Punct`s).
pub fn name<S>(mut tokens: &[TokenTree<S>]) -> (Option<String>, &[TokenTree<S>])
where
    S: TokenStream,
{
    // Try to parse the first ident.
    let mut result = match &tokens[0] {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => return (None, tokens),
    };

    tokens = &tokens[1..];

    while tokens.len() >= 2 {
        match &tokens[0] {
            TokenTree::Punct(punct) if punct.as_char() == '-' => {}
            _ => break,
        };

        match &tokens[1] {
            TokenTree::Ident(ident) => {
                result += "-";
                result += &ident.to_string();
                tokens = &tokens[2..];
            }
            _ => break,
        }
    }

    (Some(result), tokens)
}
