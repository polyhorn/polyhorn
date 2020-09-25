use super::scanner::{Ident, TokenTree};

/// This trait should be implemented by domain-specific languages. Specifically,
/// Stylo doesn't parse selectors or properties: it simply delegates that work to
/// the domain-specific driver.
pub trait Driver {
    type Selector;
    type Property;
    type Error;

    /// This function should read tokens from the given scanner and return a
    /// selector (or an error). If this function does not exhaust the scanner, it
    /// will be called again until the scanner is exhausted (i.e. all tokens have
    /// been read).
    fn parse_selector<S>(&self, scanner: &mut S) -> Result<Self::Selector, Self::Error>
    where
        S: Iterator<Item = TokenTree>;

    /// This function should read tokens from the given scanner and return a
    /// property. If no property exists with the given name, it may consume the
    /// scanner and return an error instead. If this function returns without
    /// exhausting the scanner, all remaining tokens in the scanner will produce
    /// an unexpected token error.
    fn parse_property<S>(&self, name: Ident, value: &mut S) -> Result<Self::Property, Self::Error>
    where
        S: Iterator<Item = TokenTree>;
}
