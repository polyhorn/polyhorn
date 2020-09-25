use casco::stream::TokenStream;
use casco::{Item, StyleSheet};
use std::collections::HashMap;

use super::driver::{Driver, Property, Selector};
use super::types::Style;
use super::StyleBuilder;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Edge {
    from: Option<String>,
    to: Option<String>,
}

impl Edge {
    pub fn new(from: Option<String>, to: Option<String>) -> Edge {
        Edge { from, to }
    }

    pub fn asterisk() -> Edge {
        Edge::new(None, None)
    }

    pub fn from(&self) -> Option<&str> {
        Some(self.from.as_ref()?.as_str())
    }

    pub fn to(&self) -> Option<&str> {
        Some(self.to.as_ref()?.as_str())
    }

    pub fn presedence(&self) -> usize {
        match (&self.from, &self.to) {
            (Some(_), Some(_)) => 3,
            (None, Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
        }
    }

    pub fn without_from(&self) -> Edge {
        Edge::new(None, self.to.clone())
    }

    pub fn without_to(&self) -> Edge {
        Edge::new(self.from.clone(), None)
    }
}

#[derive(Debug)]
pub struct Automaton<S>
where
    S: TokenStream,
{
    styles: HashMap<Edge, Vec<Property<S>>>,
}

impl<S> Automaton<S>
where
    S: TokenStream,
{
    pub fn new(stylesheet: StyleSheet<Driver<S>, S>) -> Automaton<S> {
        let mut automaton = Automaton {
            styles: HashMap::new(),
        };

        automaton.build(stylesheet);

        automaton
    }

    pub fn apply(&self, builder: &mut StyleBuilder<S>, edge: &Edge, style: Style) -> Style
    where
        S: TokenStream,
    {
        if let Some(properties) = self.styles.get(edge) {
            builder.build_style(properties, style)
        } else {
            style
        }
    }

    pub fn style(&self, edge: &Edge) -> Style {
        let mut builder = StyleBuilder::new();

        let mut style = Style::default();
        style = self.apply(&mut builder, &Edge::asterisk(), style);
        style = self.apply(&mut builder, &Edge::asterisk(), style);
        style = self.apply(&mut builder, &edge.without_to(), style);
        style = self.apply(&mut builder, &edge.without_from(), style);
        style = self.apply(&mut builder, edge, style);

        // TODO: obtain the diagnostics.

        style
    }

    pub fn variants(&self) -> impl Iterator<Item = (&Edge, Style)> + '_ {
        self.styles.keys().map(move |edge| (edge, self.style(edge)))
    }

    fn build_edge(&mut self, edge: Edge, selectors: Vec<Selector>) -> Edge {
        let mut selectors = selectors.into_iter();

        let (to, from) = match selectors.next() {
            Some(Selector::Ampersand) if edge.to.is_some() => (edge.to, {
                match selectors.next() {
                    Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                        Some(from)
                    }
                    _ => None,
                }
            }),
            Some(Selector::ClassName(to)) | Some(Selector::State(to)) => (Some(to), {
                match selectors.next() {
                    Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                        Some(from)
                    }
                    _ => None,
                }
            }),
            Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                (None, Some(from))
            }
            _ => unimplemented!("TODO: emit error."),
        };

        Edge::new(from, to)
    }

    fn build_items(&mut self, edge: Edge, items: Vec<Item<Driver<S>, S>>) {
        let mut properties = vec![];

        for item in items {
            match item {
                Item::Property(property) => properties.push(property),
                Item::Rule(rule) => {
                    let edge = self.build_edge(edge.clone(), rule.selectors);
                    self.build_items(edge, rule.items);
                }
            }
        }

        self.styles.entry(edge).or_default().extend(properties);
    }

    fn build(&mut self, parse: StyleSheet<Driver<S>, S>) {
        self.build_items(Edge::asterisk(), parse.items);
    }
}
