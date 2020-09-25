mod image;
mod keyboard_avoiding_view;
mod modal;
mod scroll_view;
mod status_bar;
mod text;
mod text_input;
mod view;
mod window;

pub use image::Image;
pub use keyboard_avoiding_view::KeyboardAvoidingView;
pub use modal::Modal;
pub use scroll_view::{ScrollView, ScrollViewIndicatorStyle};
pub use status_bar::{StatusBar, StatusBarStyle};
pub use text::Text;
pub use text_input::TextInput;
pub use view::{View, ViewHandle};
pub use window::{SafeAreaInsets, UseSafeAreaInsets, Window};

use std::sync::Arc;

pub struct Callback<T>(Arc<dyn Fn(T) + Send>);

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Callback(self.0.clone())
    }
}

impl<F, T> From<F> for Callback<T>
where
    F: Fn(T) -> () + Send + 'static,
{
    fn from(value: F) -> Self {
        Callback(Arc::new(value))
    }
}
