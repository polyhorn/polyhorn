use polyhorn::*;

pub struct NavigationBar {
    pub style: Style,
}

impl Component for NavigationBar {
    fn render(&self, manager: &mut Manager) -> Element {
        let insets = use_safe_area_insets!(manager);

        let height = self.style.height.unwrap_or(44.px());

        poly!(<View style={ style! {
            flex_shrink: 0.0;
            background_color: self.style.background_color.clone();
            padding: (insets.top.px(), 0.px(), 0.px(), 0.px());
        } } ...>
            <View style={ style! {
                height: height;
                flex_shrink: 0.0;
            } } ...>
                { manager.children() }
            </View>
        </View>)
    }
}
