use polyhorn::Component;

mod container;
mod navigator;

pub use container::ModalContainer;
pub use navigator::Navigator;

pub trait Screen: Clone + Component + Default + 'static {}

#[derive(Copy, Clone, Default)]
pub struct ModalContext;
