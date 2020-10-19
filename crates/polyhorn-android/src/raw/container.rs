use as_any::AsAny;
use polyhorn_android_sys::{Activity, View};
use polyhorn_ui::layout::LayoutNode;

use super::{Environment, Platform};

/// Concrete implementation of an Android-specific container.
pub trait Container: AsAny + Send {
    /// This function should mount the given child container onto this container.
    fn mount(&mut self, child: &mut OpaqueContainer, environment: &mut Environment);

    /// This function should unmount this container from its parent container.
    fn unmount(&mut self);

    fn to_view(&self) -> Option<View> {
        None
    }
}

/// Opaque wrapper around a container with two optional layouts attached. If
/// both are given, the first refers to the container's layout itself, whereas
/// the second refers to the container's content layout. These can be different
/// when working with scroll views for example, which are essentially treated
/// as two adjacent nodes in the layout tree.
pub struct OpaqueContainer(Option<LayoutNode>, Option<LayoutNode>, Box<dyn Container>);

impl OpaqueContainer {
    /// Returns a new opaque container with the given layout, view and
    /// optionally a separate content layout.
    pub fn new<T>(
        layout: LayoutNode,
        content_layout: Option<LayoutNode>,
        view: T,
    ) -> OpaqueContainer
    where
        T: Container,
    {
        OpaqueContainer(Some(layout), content_layout, Box::new(view))
    }

    pub unsafe fn activity(
        env: *mut std::ffi::c_void,
        object: *mut std::ffi::c_void,
    ) -> OpaqueContainer {
        OpaqueContainer(
            None,
            None,
            Box::new(Activity::with_env(env as _, object as _)),
        )
    }

    /// Returns the layout of this container (if applicable).
    pub fn layout(&self) -> Option<&LayoutNode> {
        self.0.as_ref()
    }

    /// Returns the content layout of this container (if applicable). Returns
    /// `None` if not applicable, even if the container has a regular layout.
    pub fn content_layout(&self) -> Option<&LayoutNode> {
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
    fn mount(&mut self, container: &mut OpaqueContainer, environment: &mut Environment) {
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

        self.2.mount(container, environment);
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
