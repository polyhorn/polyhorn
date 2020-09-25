use super::Layout;
use super::Platform;
use as_any::AsAny;
use polyhorn_ios_sys as sys;

pub trait Container: AsAny + 'static {
    fn mount(&mut self, child: &mut OpaqueContainer);
    fn unmount(&mut self);

    fn to_view(&self) -> Option<sys::UIView> {
        None
    }

    fn to_view_controller(&self) -> Option<sys::UIViewController> {
        None
    }

    fn to_window(&self) -> Option<sys::UIWindow> {
        None
    }
}

pub struct OpaqueContainer(Option<Layout>, Option<Layout>, Box<dyn Container>);

impl OpaqueContainer {
    pub fn new<T>(layout: Layout, content_layout: Option<Layout>, view: T) -> OpaqueContainer
    where
        T: Container,
    {
        OpaqueContainer(Some(layout), content_layout, Box::new(view))
    }

    pub fn root() -> OpaqueContainer {
        OpaqueContainer(None, None, Box::new(sys::UIApplication::shared()))
    }

    pub fn layout(&self) -> Option<&Layout> {
        self.0.as_ref()
    }

    pub fn content_layout(&self) -> Option<&Layout> {
        self.1.as_ref()
    }

    pub fn container(&self) -> &dyn Container {
        self.2.as_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        self.2.as_mut().as_any_mut().downcast_mut::<T>()
    }
}

impl polyhorn_core::Container<Platform> for OpaqueContainer {
    fn mount(&mut self, container: &mut OpaqueContainer) {
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
