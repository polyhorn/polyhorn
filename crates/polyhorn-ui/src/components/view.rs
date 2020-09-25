use polyhorn_core::Reference;

use crate::events::EventListener;
use crate::handles::{Imperative, ViewHandle};
use crate::styles::ViewStyle;

/// The base component.
pub struct View<H>
where
    H: ViewHandle,
{
    /// Controls the appearance and layout of a View.
    pub style: ViewStyle,

    /// Called when the user cancels pressing a View.
    pub on_pointer_cancel: EventListener<()>,

    /// Called when the user starts pressing a View.
    pub on_pointer_down: EventListener<()>,

    /// Called when the user stops pressing a View.
    pub on_pointer_up: EventListener<()>,

    /// This is a reference to an imperative view handle that can be used to
    /// measure the dimensions of this view or schedule animations.
    pub reference: Reference<H>,
}

impl<H> Default for View<H>
where
    H: ViewHandle,
{
    fn default() -> Self {
        View {
            style: Default::default(),
            on_pointer_cancel: Default::default(),
            on_pointer_down: Default::default(),
            on_pointer_up: Default::default(),
            reference: Default::default(),
        }
    }
}

impl<H> Imperative for View<H>
where
    H: ViewHandle,
{
    type Handle = H;
}
