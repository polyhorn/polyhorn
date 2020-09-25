use super::driver::{Driver, Property, Selector};
use super::types::Style;
use casco::{Item, Parse};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Edge<'a> {
    from: Option<&'a str>,
    to: Option<&'a str>,
}

impl<'a> Edge<'a> {
    pub fn new(from: Option<&'a str>, to: Option<&'a str>) -> Edge<'a> {
        Edge { from, to }
    }

    pub fn asterisk() -> Edge<'a> {
        Edge::new(None, None)
    }

    pub fn from(&self) -> Option<&'a str> {
        self.from
    }

    pub fn to(&self) -> Option<&'a str> {
        self.to
    }

    pub fn presedence(&self) -> usize {
        match (&self.from, &self.to) {
            (Some(_), Some(_)) => 3,
            (None, Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
        }
    }

    pub fn without_from(&self) -> Edge<'a> {
        Edge::new(None, self.to)
    }

    pub fn without_to(&self) -> Edge<'a> {
        Edge::new(self.from, None)
    }
}

#[derive(Debug)]
pub struct Automaton<'a> {
    styles: HashMap<Edge<'a>, Vec<&'a Property>>,
}

impl<'a> Automaton<'a> {
    pub fn new(parse: &'a Parse<Driver>) -> Automaton<'a> {
        let mut automaton = Automaton {
            styles: HashMap::new(),
        };

        automaton.build(&parse);

        automaton
    }

    pub fn apply(&self, edge: Edge, style: &mut Style) {
        if let Some(properties) = self.styles.get(&edge) {
            for property in properties {
                property.apply(style)
            }
        }
    }

    pub fn style(&self, edge: Edge) -> Style {
        let mut style = Style::default();

        self.apply(Edge::asterisk(), &mut style);
        self.apply(edge.without_to(), &mut style);
        self.apply(edge.without_from(), &mut style);
        self.apply(edge, &mut style);

        style
    }

    pub fn variants(&self) -> impl Iterator<Item = (Edge<'a>, Style)> + '_ {
        self.styles
            .keys()
            .copied()
            .map(move |edge| (edge, self.style(edge)))
    }

    fn build_edge(&mut self, edge: Edge<'a>, selectors: &'a [Selector]) -> Edge<'a> {
        let mut selectors = selectors.into_iter();

        let (to, from) = match selectors.next() {
            Some(Selector::Ampersand) if edge.to.is_some() => (edge.to, {
                match selectors.next() {
                    Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                        Some(from.as_str())
                    }
                    _ => None,
                }
            }),
            Some(Selector::ClassName(to)) | Some(Selector::State(to)) => (Some(to.as_str()), {
                match selectors.next() {
                    Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                        Some(from.as_str())
                    }
                    _ => None,
                }
            }),
            Some(Selector::FromClassName(from)) | Some(Selector::FromState(from)) => {
                (None, Some(from.as_str()))
            }
            _ => unimplemented!("TODO: emit error."),
        };

        Edge::new(from, to)
    }

    fn build_items(&mut self, edge: Edge<'a>, items: &'a [Item<Driver>]) {
        let mut properties = vec![];

        for item in items {
            match item {
                Item::Property(property) => properties.push(property),
                Item::Rule(rule) => {
                    let edge = self.build_edge(edge, &rule.selectors);
                    self.build_items(edge, &rule.items);
                }
            }
        }

        self.styles.entry(edge).or_default().extend(properties);
    }

    fn build(&mut self, parse: &'a Parse<Driver>) {
        self.build_items(Edge::asterisk(), &parse.items);
    }
}
