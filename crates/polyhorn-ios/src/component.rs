use super::{Element, Manager, Platform};
use as_any::AsAny;
use std::rc::Rc;

pub trait Component: AsAny {
    fn render(&self, manager: &mut Manager) -> Element;
}

#[derive(Clone)]
pub struct OpaqueComponent(Rc<dyn Component>);

impl AsRef<dyn Component> for OpaqueComponent {
    fn as_ref(&self) -> &dyn Component {
        self.0.as_ref()
    }
}

/// This is a little bit of machinery that is necessary until we have proper
/// trait aliases in Rust. Ideally, we would want to be able to write just:
/// `poly_ios::Component == poly_core::Component<poly_ios::Platform>`, but that's
/// not yet possible.
mod machinery {
    use super::{Component, Element, Manager, OpaqueComponent, Platform, Rc};

    impl polyhorn_core::Component<Platform> for OpaqueComponent {
        fn render(&self, manager: &mut Manager) -> Element {
            self.0.render(manager)
        }
    }

    impl<T> From<T> for OpaqueComponent
    where
        T: Component + 'static,
    {
        fn from(value: T) -> Self {
            OpaqueComponent(Rc::new(value))
        }
    }
}
