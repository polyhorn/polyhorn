use itertools::Itertools;

use super::scanner::{Delimiter, Group, TokenTree};
use super::{Driver, Item, Parse, Rule};

pub struct Parser<'a, D>
where
    D: Driver + ?Sized,
{
    driver: &'a D,
    errors: Vec<D::Error>,
}

impl<'a, D> Parser<'a, D>
where
    D: Driver,
{
    /// This function returns a new parser with the given driver.
    pub fn new(driver: &'a D) -> Parser<'a, D> {
        Parser {
            driver,
            errors: vec![],
        }
    }

    fn parse_property<S>(&mut self, mut preamble: S) -> Option<D::Property>
    where
        S: Iterator<Item = TokenTree>,
    {
        let name = match preamble.next() {
            Some(TokenTree::Ident(ident)) => ident,
            _ => unreachable!(),
        };

        let _colon = match preamble.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => punct,
            _ => unreachable!(),
        };

        let result = self.driver.parse_property(name, &mut preamble);

        // TODO: print an error if the preamble is not empty. In the future,
        // this might also be a suitable place to implement support for
        // importance specifiers (i.e. `!important`).
        if result.is_ok() {
            assert!(preamble.next().is_none());
        }

        match result {
            Ok(property) => Some(property),
            Err(error) => {
                self.errors.push(error);
                None
            }
        }
    }

    fn parse_selector<S>(&mut self, mut preamble: S) -> Option<D::Selector>
    where
        S: Iterator<Item = TokenTree>,
    {
        match self.driver.parse_selector(&mut preamble) {
            Ok(selector) => Some(selector),
            Err(error) => {
                self.errors.push(error);
                None
            }
        }
    }

    fn parse_rule<S>(&mut self, preamble: S, group: Group) -> Option<Rule<D>>
    where
        S: Iterator<Item = TokenTree>,
    {
        let mut selectors = vec![];

        let mut preamble = preamble.peekable();

        while preamble.peek().is_some() {
            if let Some(selector) = self.parse_selector(&mut preamble) {
                selectors.push(selector);
            }
        }

        Some(Rule {
            selectors,
            items: self.parse_block(group.stream().into_iter()),
        })
    }

    fn parse_item<S>(&mut self, preamble: S, terminator: TokenTree) -> Option<Item<D>>
    where
        S: Iterator<Item = TokenTree>,
    {
        match terminator {
            TokenTree::Punct(punct) if punct.as_char() == ';' => self
                .parse_property(preamble.into_iter())
                .map(|prop| Item::Property(prop)),
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => self
                .parse_rule(preamble, group)
                .map(|rule| Item::Rule(rule)),
            _ => unimplemented!(),
        }
    }

    fn parse_block<S>(&mut self, mut scanner: S) -> Vec<Item<D>>
    where
        S: Clone + Iterator<Item = TokenTree>,
    {
        let mut items = vec![];

        loop {
            // We parse a single preamble. A line ends before either an `;` punct or
            // a `{}` group. More explicitly: the line does not include that
            // terminator (mostly because of how `take_while` works).
            let preamble = scanner
                .take_while_ref(|tree| match tree {
                    TokenTree::Punct(punct) if punct.as_char() == ';' => false,
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => false,
                    _ => true,
                })
                .collect::<Vec<_>>();

            match scanner.next() {
                Some(terminator) => {
                    if let Some(item) = self.parse_item(preamble.into_iter(), terminator) {
                        items.push(item);
                    }
                }
                None => match preamble.len() {
                    0 => break,
                    _ => {
                        // TODO: pretend that it is a property with a missing `;`
                        // and report an error.
                        unimplemented!(
                            "Parsing an unterminated preamble: {:?} is not yet supported.",
                            preamble
                        );
                    }
                },
            }
        }

        items
    }

    pub fn parse<S>(mut self, scanner: S) -> Parse<D>
    where
        S: Clone + Iterator<Item = TokenTree>,
    {
        Parse {
            items: self.parse_block(scanner),
            errors: self.errors,
        }
    }
}
