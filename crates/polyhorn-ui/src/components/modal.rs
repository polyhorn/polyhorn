use crate::events::EventListener;

/// Renders its children in a system-provided modal window.
#[derive(Clone)]
pub struct Modal {
    /// Controls the visibility of this modal. Changing this value will animate
    /// the visibility of the modal window with a system-provided transition.
    /// Note that is preferred over unmounting the modal entirely, at least
    /// until the `on_dismiss` event is emitted, because the modal will not be
    /// able to animate its dismissal while unmounting already.
    pub visible: bool,

    /// Event listener that is invoked after the modal is dismissed (and its
    /// animation has completed). This event will be emitted even when the
    /// modal's visibility is programmatically and directly changed through the
    /// `visible` property.
    pub on_dismiss: EventListener<()>,
}

impl Default for Modal {
    fn default() -> Self {
        Modal {
            visible: true,
            on_dismiss: Default::default(),
        }
    }
}
