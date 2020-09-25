use polyhorn::*;

use super::ItemContext;

pub struct NavigationItem {
    pub on_back_press: EventListener<()>,
    pub left: Element,
    pub title: Element,
    pub right: Element,
    pub tint_color: Color,
    pub title_style: TextStyle,
}

yoyo::yoyo!(States, {
    opacity_transition: ease_in_out;

    :initial {
        opacity: 0.0;
    }

    .rest {
        opacity: 1.0;
    }

    :exit {
        opacity: 0.0;
    }
});

impl Component for NavigationItem {
    fn render(&self, manager: &mut Manager) -> Element {
        let presence: yoyo::Presence<ItemContext> = yoyo::use_presence!(manager);

        poly!(<yoyo::View::<States> variant={ States::Rest } style={ style! {
            flex_grow: 1.0;
            flex_direction: FlexDirection::Row;
            justify_content: JustifyContent::Center;
        } } presence={ presence.into_dyn() } ...>
            <View style={ style! {
                flex_direction: FlexDirection::Row;
                flex_shrink: 0.0;
                flex_grow: 1.0;
                flex_basis: 32.px();
                justify_content: JustifyContent::FlexStart;
            } } ...>
                { if self.on_back_press.is_some() {
                    Some(poly!(<yoyo::TouchableOpacity style={ style! {
                        flex_direction: FlexDirection::Row;
                        align_items: AlignItems::Center;
                        padding: (0.px(), 12.px());
                    } } on_pointer_up={ self.on_back_press.clone() } ...>
                        <Image source={ ImageSource::with_name("polyhorn-navigation/back").unwrap() }
                           tint_color={ self.tint_color.clone() } />
                        <View style={ style! {
                            width: 6.px();
                        } } ... />
                        <Text style={ text_style! {
                            font:Font::system_font(17.0);
                            color: self.tint_color.clone();
                        } }>
                            "Back"
                        </Text>
                    </yoyo::TouchableOpacity>))
                } else {
                    None
                } }
            </View>
            <View style={ style! {
                flex_direction: FlexDirection::Row;
                align_items: AlignItems::Center;
                justify_content: JustifyContent::Center;
                flex_grow: 1.0;
                flex_basis: Dimension::auto();
            } } ...>
                { match self.title {
                    Element::String(_) => poly!(
                        <Text style={ TextStyle {
                            alignment: TextAlignment::Center,
                            ..self.title_style.clone()
                        } }>
                            { self.title.clone() }
                        </Text>
                    ),
                    _ => self.title.clone(),
                } }
            </View>
            <View style={ style! {
                flex_direction: FlexDirection::Row;
                flex_shrink: 0.0;
                flex_grow: 1.0;
                flex_basis: 32.px();
                justify_content: JustifyContent::FlexEnd;
            } } ...>
                { match self.right {
                    Element::String(_) => poly!(
                        <yoyo::TouchableOpacity style={ style! {
                            flex_direction: FlexDirection::Row;
                            align_items: AlignItems::Center;
                            padding: (0.px(), 12.px());
                        } } ...>
                            <Text style={ text_style! {
                                font:Font::bold_system_font(17.0);
                                color: self.tint_color.clone();
                            } }>
                                { self.right.clone() }
                            </Text>
                        </yoyo::TouchableOpacity>
                    ),
                    _ => self.right.clone(),
                } }
            </View>
        </yoyo::View::<States>>)
    }
}
