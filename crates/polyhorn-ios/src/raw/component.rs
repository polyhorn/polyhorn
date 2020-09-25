use as_any::AsAny;
use std::rc::Rc;

use super::Platform;
use crate::{Element, Manager};

/// Platform-specific component trait.
pub trait Component: AsAny {
    /// Render function that must be implemented by components.
    fn render(&self, manager: &mut Manager) -> Element;
}

/// Opaque reference counted wrapper around a component.
#[derive(Clone)]
pub struct OpaqueComponent(Rc<dyn Component>);

impl AsRef<dyn Component> for OpaqueComponent {
    fn as_ref(&self) -> &dyn Component {
        self.0.as_ref()
    }
}

/// This is a little bit of machinery that is necessary until we have proper
/// trait aliases in Rust. Ideally, we would be able to alias
/// `polyhorn_ios::Component` to
/// `polyhorn_core::Component<polyhorn_ios::Platform>`, but that's not yet
/// possible.
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
