use polyhorn::prelude::*;
use polyhorn_ui::assets::ImageSource;
use polyhorn_ui::color::Color;
use polyhorn_ui::events::EventListener;
use polyhorn_ui::styles::{ImageStyle, Inherited, TextAlign, TextStyle};

use super::ItemContext;

#[derive(Default)]
pub struct NavigationItem {
    pub on_back_press: EventListener<()>,
    pub left: Element,
    pub title: Element,
    pub right: Element,
    pub tint_color: Color,
    pub title_style: TextStyle,
}

yoyo::yoyo!(States, {
    transition-opacity: ease-in-out(0.4);

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
            flex-grow: 1.0;
            flex-direction: row;
            justify-content: center;
        } } presence={ presence.into_dyn() }>
            <View style={ style! {
                flex-direction: row;
                flex-shrink: 0.0;
                flex-grow: 1.0;
                flex-basis: 32px;
                justify-content: flex-start;
            } }>
                { if self.on_back_press.is_some() {
                    Some(poly!(<yoyo::TouchableOpacity style={ style! {
                        flex-direction: row;
                        align-items: center;
                        padding: 0px 12px;
                    } } on_pointer_up={ self.on_back_press.clone() }>
                        <Image source={ ImageSource::Asset(asset!("back")) }
                                style={ ImageStyle {
                            tint_color: Some(self.tint_color),
                            ..Default::default()
                        } } />
                        <View style={ style! {
                            width: 6px;
                        } } />
                        <Text style={ TextStyle {
                                color: Inherited::Specified(self.tint_color),
                            ..style! {
                                font-size: 17px;
                            }
                        } }>
                            "Back"
                        </Text>
                    </yoyo::TouchableOpacity>))
                } else {
                    None
                } }
            </View>
            <View style={ style! {
                flex-direction: row;
                align-items: center;
                justify-content: center;
                flex-grow: 1.0;
                flex-basis: auto;
            } }>
                { match self.title {
                    Element::String(_) => poly!(
                        <Text style={ TextStyle {
                            text_align: Inherited::Specified(TextAlign::Center),
                            ..self.title_style
                        } }>
                            { self.title.clone() }
                        </Text>
                    ),
                    _ => self.title.clone(),
                } }
            </View>
            <View style={ style! {
                flex-direction: row;
                flex-shrink: 0.0;
                flex-grow: 1.0;
                flex-basis: 32px;
                justify-content: flex-end;
            } }>
                { match self.right {
                    Element::String(_) => poly!(
                        <yoyo::TouchableOpacity style={ style! {
                            flex-direction: row;
                            align-items: center;
                            padding: 0px 12px;
                        } }>
                            <Text style={ TextStyle {
                                color: Inherited::Specified(self.tint_color),
                                ..style! {
                                    font-weight: bold;
                                    font-size: 17px;
                                }
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
