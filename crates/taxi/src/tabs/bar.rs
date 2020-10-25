use polyhorn::prelude::*;
use polyhorn_ui::color::Color;
use polyhorn_ui::events::EventListener;
use polyhorn_ui::styles::{Inherited, TextStyle, ViewStyle};

#[derive(Default)]
pub struct TabBar {}

impl Component for TabBar {
    fn render(&self, manager: &mut Manager) -> Element {
        poly!(<View style={ ViewStyle {
            background_color: Color::rgba(247, 247, 247, 0.8),
            ..style! {
                flex-shrink: 0.0;
                flex-direction: row;
                height: 83px;
                padding: 0px 0px 24px 0px;
            }
        } }>
            { manager.children() }
        </View>)
    }
}

#[derive(Default)]
pub struct TabBarItem {
    pub selected: bool,
    pub title: String,
    pub on_press: EventListener<()>,
}

impl Component for TabBarItem {
    fn render(&self, _manager: &mut Manager) -> Element {
        let tint = if self.selected {
            Color::hex(0x007AFF)
        } else {
            Color::hex(0x8E8E93)
        };

        poly!(<View style={ style! {
            flex-grow: 1.0;
            align-items: center;
            justify-content: center;
        } } on_pointer_up={ self.on_press.clone() }>
            <View style={ ViewStyle {
                background_color: tint,
                ..style! {
                    // border_radius: 2.px();
                    width: 28px;
                    height: 28px;
                    margin: 0px 0px 2px 0px;
                }
            } } />
            <Text style={ TextStyle {
                color: Inherited::Specified(tint),
                ..style! {
                    font-size: 10px;
                }
            } }>
                { self.title.clone() }
            </Text>
        </View>)
    }
}
