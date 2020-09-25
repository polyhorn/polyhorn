use crate::animation::Animatable;
use crate::geometry::Size;
use crate::layout::LayoutGuide;

/// This trait is implemented by platform-specific View handles.
pub trait ViewHandle: Animatable {
    /// This function should return the layout guides of this view.
    /// TODO: on what thread does this occur?
    fn layout_guide(&self) -> LayoutGuide<f32>;

    /// This function should call the given callback with the size of this view.
    /// Explicitly passing a command buffer ensures that the dimensions are
    /// measured in the same UI event loop iteration as the current render
    /// itself.
    fn size_with_buffer<F>(&mut self, buffer: &mut Self::CommandBuffer, callback: F)
    where
        F: FnOnce(Size<f32>) + Send + 'static;
}
