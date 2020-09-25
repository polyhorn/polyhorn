use polyhorn::*;

pub struct TabBar {}

impl Component for TabBar {
    fn render(&self, manager: &mut Manager) -> Element {
        poly!(<View style={ style! {
            flex_shrink: 0.0;
            flex_direction: FlexDirection::Row;
            background_color: Color::from_rgba(247, 247, 247, 0.8);
            height: 83.px();
            padding: (0.px(), 0.px(), 24.px(), 0.px());
        } } ...>
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
            Color::from_hex(0x007AFF)
        } else {
            Color::from_hex(0x8E8E93)
        };

        poly!(<View style={ style! {
            flex_grow: 1.0;
            align_items: AlignItems::Center;
            justify_content: JustifyContent::Center;
        } } on_pointer_up={ self.on_press.clone() } ...>
            <View style={ style! {
                background_color: tint.clone();
                border_radius: 2.px();
                width: 28.px();
                height: 28.px();
                margin: (0.px(), 0.px(), 2.px(), 0.px());
            } } ... />
            <Text style={ text_style! {
                font: Font::system_font(10.0);
                color: tint.clone();
            } }>
                { self.title.clone() }
            </Text>
        </View>)
    }
}
