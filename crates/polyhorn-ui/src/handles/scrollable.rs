/// This trait is implemented by platform-specific Scrollable handles.
pub trait ScrollableHandle {
    /// Momentarily displays the scroll indicators (if applicable).
    fn flash_scroll_indicators(&mut self);
}
