use polyhorn_ios_sys::polykit::{PLYScrollView, PLYView};
use polyhorn_ui::styles::{ScrollableStyle, Transform, ViewStyle, Visibility};

use crate::raw::convert::Convert;

/// This trait is implemented for every style that can be applied to an instance
/// of `UIView`.
pub trait Apply<T> {
    /// Applies this style to the given view.
    fn apply(&self, to: &mut T);
}

impl Apply<PLYView> for ViewStyle {
    fn apply(&self, view: &mut PLYView) {
        view.set_background_color(self.background_color.convert());
        view.set_alpha(self.opacity as _);
        view.set_corner_radii(self.border_radius.convert());
        view.set_hidden(self.visibility == Visibility::Hidden);
        view.set_transform(Transform::squash(self.transform, Default::default()).convert());
    }
}

impl Apply<PLYScrollView> for ScrollableStyle {
    fn apply(&self, view: &mut PLYScrollView) {
        view.set_scroll_padding(self.scroll_padding.convert());
        view.set_indicator_style(self.scrollbar_color.convert());
        view.set_scrollbar_padding(self.scrollbar_padding.convert());
    }
}
