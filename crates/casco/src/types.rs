use std::fmt::{Debug, Formatter, Result};

use super::Driver;

/// A rule consists of a zero or more selectors and zero or more sub-items. These
/// sub-items in turn can be properties or nested rules.
pub struct Rule<D>
where
    D: Driver + ?Sized,
{
    /// These are the selectors of a rule.
    pub selectors: Vec<D::Selector>,

    /// These are the sub-items in a rule.
    pub items: Vec<Item<D>>,
}

impl<D> Debug for Rule<D>
where
    D: Driver + ?Sized,
    D::Selector: Debug,
    D::Property: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Rule")
            .field("selectors", &self.selectors)
            .field("items", &self.items)
            .finish()
    }
}

/// This is a single item. An item consists of a single line that is delimited
/// either with a `;` punct (property) or a `{ ... }` group (rule).
pub enum Item<D>
where
    D: Driver + ?Sized,
{
    /// This is a rule `... { ... }`.
    Rule(Rule<D>),

    /// This is a property `...: ...;`.
    Property(D::Property),
}

impl<D> Item<D>
where
    D: Driver + ?Sized,
{
    /// This function returns a boolean that indicates if this item is a rule.
    pub fn is_rule(&self) -> bool {
        match self {
            Item::Rule(_) => true,
            Item::Property(_) => false,
        }
    }

    /// This function returns a boolean that indicates if this item is a
    /// property.
    pub fn is_property(&self) -> bool {
        match self {
            Item::Rule(_) => false,
            Item::Property(_) => true,
        }
    }
}

impl<D> Debug for Item<D>
where
    D: Driver + ?Sized,
    D::Selector: Debug,
    D::Property: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Item::Rule(rule) => f.debug_tuple("Item::Rule").field(rule).finish(),
            Item::Property(property) => f.debug_tuple("Item::Property").field(property).finish(),
        }
    }
}

pub struct Parse<D>
where
    D: Driver + ?Sized,
{
    pub items: Vec<Item<D>>,
    pub errors: Vec<D::Error>,
}

impl<D> Debug for Parse<D>
where
    D: Driver + ?Sized,
    D::Selector: Debug,
    D::Property: Debug,
    D::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Parse")
            .field("items", &self.items)
            .field("errors", &self.errors)
            .finish()
    }
}
