use casco::stream::TokenStream;

use super::{Property, PropertyValue};

/// Type of style that a property belongs to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StyleKind {
    /// Refers to properties that belong to `ImageStyle`.
    Image,

    /// Refers to properties that belong to `ScrollableStyle`.
    Scrollable,

    /// Refers to properties that belong to `TextStyle`.
    Text,

    /// Refers to properties that belong to `ViewStyle`.
    View,
}

impl StyleKind {
    /// Infers the type of style that a property belongs to based on its value.
    /// This will always succeed because within Polyhorn UI, every property
    /// corresponds to one type of style.
    pub fn infer<S>(property: &Property<S>) -> StyleKind
    where
        S: TokenStream,
    {
        match &property.value {
            PropertyValue::AlignItems(_) => StyleKind::View,
            PropertyValue::BackgroundColor(_) => StyleKind::View,
            PropertyValue::Border(_) => StyleKind::View,
            PropertyValue::BorderRadius(_) => StyleKind::View,
            PropertyValue::Bottom(_) => StyleKind::View,
            PropertyValue::Color(_) => StyleKind::Text,
            PropertyValue::Direction(_) => StyleKind::View,
            PropertyValue::FlexBasis(_) => StyleKind::View,
            PropertyValue::FlexDirection(_) => StyleKind::View,
            PropertyValue::FlexGrow(_) => StyleKind::View,
            PropertyValue::FlexShrink(_) => StyleKind::View,
            PropertyValue::FontFamily(_) => StyleKind::Text,
            PropertyValue::FontSize(_) => StyleKind::Text,
            PropertyValue::FontStyle(_) => StyleKind::Text,
            PropertyValue::FontWeight(_) => StyleKind::Text,
            PropertyValue::Height(_) => StyleKind::View,
            PropertyValue::JustifyContent(_) => StyleKind::View,
            PropertyValue::Left(_) => StyleKind::View,
            PropertyValue::Margin(_) => StyleKind::View,
            PropertyValue::MaxHeight(_) => StyleKind::View,
            PropertyValue::MaxWidth(_) => StyleKind::View,
            PropertyValue::MinHeight(_) => StyleKind::View,
            PropertyValue::MinWidth(_) => StyleKind::View,
            PropertyValue::ObjectFit(_) => StyleKind::Image,
            PropertyValue::Opacity(_) => StyleKind::View,
            PropertyValue::Overflow(_) => StyleKind::View,
            PropertyValue::Padding(_) => StyleKind::View,
            PropertyValue::Position(_) => StyleKind::View,
            PropertyValue::Right(_) => StyleKind::View,
            PropertyValue::TextAlign(_) => StyleKind::Text,
            PropertyValue::TintColor(_) => StyleKind::Image,
            PropertyValue::Top(_) => StyleKind::View,
            PropertyValue::Transform(_) => StyleKind::View,
            PropertyValue::Visibility(_) => StyleKind::View,
            PropertyValue::Width(_) => StyleKind::View,
        }
    }
}

/// Type of compound style that is inferred based on the individual types of
/// styles that each property belongs to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StyleCompound {
    /// Refers to properties that belong to `ImageStyle`.
    Image,

    /// Refers to properties that belong to `ImageViewStyle`.
    ImageView,

    /// Refers to properties that belong to `ScrollableStyle`.
    Scrollable,

    /// Refers to properties that belong to `ScrollableViewStyle`.
    ScrollableView,

    /// Refers to properties that belong to `TextStyle`.
    Text,

    /// Refers to properties that belong to `TextViewStyle`.
    TextView,

    /// Refers to properties that belong to `ViewStyle`.
    View,

    /// Refers to properties that belong to no style (i.e. when the set of
    /// individual types of styles is empty).
    Unit,
}

impl StyleCompound {
    /// Infers the type of compound style that makes up a given set of
    /// individual style kinds. If the individual style kinds are incompatible,
    /// this function will return `None`.
    pub fn infer<I>(kinds: I) -> Option<StyleCompound>
    where
        I: IntoIterator<Item = StyleKind>,
    {
        let kinds = kinds.into_iter();

        let mut specialization = StyleKind::View;
        let mut view = false;

        for kind in kinds {
            match kind {
                StyleKind::View => view = true,
                kind if specialization == kind => {}
                kind if specialization == StyleKind::View => specialization = kind,
                _ => return None,
            }
        }

        Some(match (specialization, view) {
            (StyleKind::Image, false) => StyleCompound::Image,
            (StyleKind::Image, true) => StyleCompound::ImageView,
            (StyleKind::Scrollable, false) => StyleCompound::Scrollable,
            (StyleKind::Scrollable, true) => StyleCompound::ScrollableView,
            (StyleKind::Text, false) => StyleCompound::Text,
            (StyleKind::Text, true) => StyleCompound::TextView,
            (StyleKind::View, false) => StyleCompound::Unit,
            (StyleKind::View, true) => StyleCompound::View,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{StyleCompound, StyleKind};

    #[test]
    fn test_compound() {
        assert_eq!(StyleCompound::infer(vec![]), Some(StyleCompound::Unit));

        assert_eq!(
            StyleCompound::infer(vec![StyleKind::View]),
            Some(StyleCompound::View)
        );

        assert_eq!(
            StyleCompound::infer(vec![StyleKind::View, StyleKind::Text]),
            Some(StyleCompound::TextView)
        );

        assert_eq!(
            StyleCompound::infer(vec![StyleKind::Image, StyleKind::View, StyleKind::Image]),
            Some(StyleCompound::ImageView)
        );
    }
}
