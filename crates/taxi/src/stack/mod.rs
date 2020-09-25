mod bar;
mod container;
mod item;
mod navigator;

pub use bar::NavigationBar;
pub use container::ScreenContainer;
pub use item::NavigationItem;
pub use navigator::Navigator;

use polyhorn::{Component, Element};

pub trait Screen: Clone + Component + Default {
    fn render_header_title(&self) -> Element;

    fn render_header_right(&self) -> Element {
        Element::empty()
    }
}

#[derive(Copy, Clone, Default)]
pub struct ItemContext;

#[derive(Copy, Clone, Default)]
pub struct ContainerContext;
