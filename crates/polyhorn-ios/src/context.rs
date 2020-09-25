use crate::*;

impl<T> Component for ContextProvider<T>
where
    T: 'static,
{
    fn render(&self, manager: &mut Manager) -> Element {
        polyhorn_core::Component::render(self, manager)
    }
}
