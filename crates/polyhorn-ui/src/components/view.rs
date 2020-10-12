use polyhorn_core::{Platform, WeakReference};

use crate::events::EventListener;
use crate::geometry::Size;
use crate::handles::{Imperative, ViewHandle};
use crate::styles::ViewStyle;

/// The base component.
pub struct View<P, H>
where
    P: Platform + ?Sized,
    H: ViewHandle + 'static,
{
    /// Controls the appearance and layout of a View.
    pub style: ViewStyle,

    /// Called when the user cancels pressing a View.
    pub on_pointer_cancel: EventListener<()>,

    /// Called when the user starts pressing a View.
    pub on_pointer_down: EventListener<()>,

    /// Called when the user stops pressing a View.
    pub on_pointer_up: EventListener<()>,

    pub on_layout: EventListener<Size<f32>>,

    /// This is a reference to an imperative view handle that can be used to
    /// measure the dimensions of this view or schedule animations.
    pub reference: Option<WeakReference<P, Option<H>>>,
}

impl<P, H> Default for View<P, H>
where
    P: Platform + ?Sized,
    H: ViewHandle,
{
    fn default() -> Self {
        View {
            style: Default::default(),
            on_pointer_cancel: Default::default(),
            on_pointer_down: Default::default(),
            on_pointer_up: Default::default(),
            on_layout: Default::default(),
            reference: Default::default(),
        }
    }
}

impl<P, H> Imperative for View<P, H>
where
    P: Platform + ?Sized,
    H: ViewHandle,
{
    type Handle = H;
}
