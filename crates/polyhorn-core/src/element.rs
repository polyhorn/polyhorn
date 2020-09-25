use super::{Builtin, Key, Platform, Reference};
use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

pub struct ElementBuiltin<P>
where
    P: Platform + ?Sized,
{
    pub key: Key,
    pub builtin: Arc<dyn Builtin<P>>,
    pub children: Box<Element<P>>,
    pub reference: Option<Reference<P::ContainerID>>,
}

impl<P> Clone for ElementBuiltin<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        ElementBuiltin {
            key: self.key.clone(),
            builtin: self.builtin.clone(),
            children: self.children.clone(),
            reference: self.reference.clone(),
        }
    }
}

pub struct ElementComponent<P>
where
    P: Platform + ?Sized,
{
    pub key: Key,
    pub component: P::Component,
    pub children: Box<Element<P>>,
}

impl<P> Clone for ElementComponent<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        ElementComponent {
            key: self.key.clone(),
            component: self.component.clone(),
            children: self.children.clone(),
        }
    }
}

pub struct ElementContext<P>
where
    P: Platform + ?Sized,
{
    pub key: Key,
    pub value: Rc<dyn Any>,
    pub children: Box<Element<P>>,
}

impl<P> Clone for ElementContext<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        ElementContext {
            key: self.key.clone(),
            value: self.value.clone(),
            children: self.children.clone(),
        }
    }
}

pub struct ElementFragment<P>
where
    P: Platform + ?Sized,
{
    pub key: Key,
    pub elements: Vec<Element<P>>,
}

impl<P> Clone for ElementFragment<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        ElementFragment {
            key: self.key.clone(),
            elements: self.elements.clone(),
        }
    }
}

pub enum Element<P>
where
    P: Platform + ?Sized,
{
    Builtin(ElementBuiltin<P>),
    Component(ElementComponent<P>),
    Context(ElementContext<P>),
    Fragment(ElementFragment<P>),
    String(String),
}

impl<P> Element<P>
where
    P: Platform + ?Sized,
{
    pub fn new(key: Key, component: P::Component, children: Element<P>) -> Element<P> {
        let children = Box::new(children);

        Element::Component(ElementComponent {
            key,
            component,
            children,
        })
    }

    pub fn builtin(
        key: Key,
        builtin: impl Builtin<P> + 'static,
        children: Element<P>,
        reference: Option<Reference<P::ContainerID>>,
    ) -> Element<P> {
        let builtin = Arc::new(builtin);
        let children = Box::new(children);

        Element::Builtin(ElementBuiltin {
            key,
            builtin,
            children,
            reference,
        })
    }

    pub fn context<T>(key: Key, value: Rc<T>, children: Element<P>) -> Element<P>
    where
        T: Any,
    {
        let children = Box::new(children);

        Element::Context(ElementContext {
            key,
            value,
            children,
        })
    }

    pub fn empty() -> Element<P> {
        // TODO: this should be a variant of the Element enum instead.
        Element::fragment(Key::new("empty"), vec![])
    }

    pub fn fragment(key: Key, elements: Vec<Element<P>>) -> Element<P> {
        Element::Fragment(ElementFragment { key, elements })
    }

    pub fn string(value: &str) -> Element<P> {
        Element::String(value.to_owned())
    }

    pub fn key(&self) -> &Key {
        match self {
            Element::Builtin(builtin) => &builtin.key,
            Element::Component(component) => &component.key,
            Element::Context(context) => &context.key,
            Element::Fragment(context) => &context.key,
            Element::String(_) => unimplemented!(),
        }
    }

    pub fn to_vec(&self) -> Vec<&Element<P>> {
        let mut results = vec![];

        match self {
            Element::Fragment(fragment) => fragment.elements.iter().for_each(|element| {
                results.extend(element.to_vec());
            }),
            _ => results.push(self),
        }

        results
    }

    pub fn at(&self, index: usize) -> Option<&Element<P>> {
        match self {
            Element::Fragment(fragment) => fragment.elements.get(index),
            _ => None,
        }
    }
}

impl<P> Clone for Element<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        match self {
            Element::Builtin(element) => Element::Builtin(element.clone()),
            Element::Component(element) => Element::Component(element.clone()),
            Element::Context(element) => Element::Context(element.clone()),
            Element::Fragment(element) => Element::Fragment(element.clone()),
            Element::String(text) => Element::String(text.clone()),
        }
    }
}

impl<P> From<Vec<Element<P>>> for Element<P>
where
    P: Platform + ?Sized,
{
    fn from(elements: Vec<Element<P>>) -> Self {
        // TODO: this element should not have a key. Instead, it should be
        // "inlined".
        Element::fragment(Key::new(()), elements)
    }
}

impl<P> From<Option<Element<P>>> for Element<P>
where
    P: Platform + ?Sized,
{
    fn from(value: Option<Element<P>>) -> Self {
        match value {
            Some(value) => value,
            None => Element::fragment(Key::new(()), vec![]),
        }
    }
}

impl<P> From<Option<&Element<P>>> for Element<P>
where
    P: Platform + ?Sized,
{
    fn from(value: Option<&Element<P>>) -> Self {
        value.cloned().into()
    }
}

impl<P> From<String> for Element<P>
where
    P: Platform + ?Sized,
{
    fn from(value: String) -> Self {
        Element::string(&value)
    }
}

impl<P> From<&str> for Element<P>
where
    P: Platform + ?Sized,
{
    fn from(value: &str) -> Self {
        Element::string(value)
    }
}
