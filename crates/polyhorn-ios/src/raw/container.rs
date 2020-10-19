use as_any::AsAny;
use polyhorn_ios_sys::polykit::{PLYView, PLYViewController, PLYWindow};
use polyhorn_ios_sys::uikit::UIApplication;

use super::{Environment, Layout, Platform};

/// Concrete implementation of a iOS-specific container.
pub trait Container: AsAny + 'static {
    /// This function should mount the given child container onto this container.
    fn mount(&mut self, child: &mut OpaqueContainer);

    /// This function should unmount this container from its parent container.
    fn unmount(&mut self);

    /// This optional function upcasts the native representation of this
    /// container to a `UIView`.
    fn to_view(&self) -> Option<PLYView> {
        None
    }

    /// This optional function upcasts the native representation of this
    /// container to a `UIViewController`.
    fn to_view_controller(&self) -> Option<PLYViewController> {
        None
    }

    /// This optional function upcasts the native representation of this
    /// container to a `UIWindow`.
    fn to_window(&self) -> Option<PLYWindow> {
        None
    }
}

/// Opaque wrapper around a container with two optional layouts attached. If
/// both are given, the first refers to the container's layout itself, whereas
/// the second refers to the container's content layout. These can be different
/// when working with scroll views for example, which are essentially treated
/// as two adjacent nodes in the layout tree.
pub struct OpaqueContainer(Option<Layout>, Option<Layout>, Box<dyn Container>);

impl OpaqueContainer {
    /// Returns a new opaque container with the given layout, view and
    /// optionally a separate content layout.
    pub fn new<T>(layout: Layout, content_layout: Option<Layout>, view: T) -> OpaqueContainer
    where
        T: Container,
    {
        OpaqueContainer(Some(layout), content_layout, Box::new(view))
    }

    /// Returns a new opaque container for the root of the UI hierarchy (which
    /// on iOS is `UIApplication`).
    pub fn root() -> OpaqueContainer {
        OpaqueContainer(None, None, Box::new(UIApplication::shared()))
    }

    /// Returns the layout of this container (if applicable).
    pub fn layout(&self) -> Option<&Layout> {
        self.0.as_ref()
    }

    /// Returns the content layout of this container (if applicable). Returns
    /// `None` if not applicable, even if the container has a regular layout.
    pub fn content_layout(&self) -> Option<&Layout> {
        self.1.as_ref()
    }

    /// Returns the container wrapped in this opaque container.
    pub fn container(&self) -> &dyn Container {
        self.2.as_ref()
    }

    /// Attempts to downcast this container to a concrete type and if
    /// successful, returns a mutable reference.
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        self.2.as_mut().as_any_mut().downcast_mut::<T>()
    }
}

impl polyhorn_core::Container<Platform> for OpaqueContainer {
    fn mount(&mut self, container: &mut OpaqueContainer, _environment: &mut Environment) {
        if let (Some(child), Some(content)) = (container.layout(), container.content_layout()) {
            let mut layouter = child.layouter().write().unwrap();
            layouter.add_child(child.node(), content.node());
        }

        if let (Some(parent), Some(child)) = (
            self.content_layout().or_else(|| self.layout()),
            container.layout(),
        ) {
            let mut layouter = parent.layouter().write().unwrap();
            layouter.add_child(parent.node(), child.node());
        }

        self.2.mount(container);
    }

    fn unmount(&mut self) {
        if let Some(layout) = self.content_layout() {
            let mut layouter = layout.layouter().write().unwrap();
            layouter.remove(layout.node());
        }

        if let Some(layout) = self.layout() {
            let mut layouter = layout.layouter().write().unwrap();
            layouter.remove(layout.node());
        }

        self.2.unmount();
    }
}
