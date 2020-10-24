use polyhorn::prelude::*;
use polyhorn_ui::events::EventListener;
use polyhorn_ui::styles::ViewStyle;

use crate as yoyo;

#[derive(Default)]
pub struct TouchableOpacity {
    pub style: ViewStyle,
    pub on_pointer_down: EventListener<()>,
    pub on_pointer_up: EventListener<()>,
}

yoyo::yoyo!(States, {
    opacity: 1.0;

    :press {
        opacity: 0.5;
    }

    // We only animate the transition between `:press` and `:initial`
    // (not vice versa).
    :from(:press) {
        transition-opacity: ease-in-out(0.4);
    }
});

impl Component for TouchableOpacity {
    fn render(&self, manager: &mut Manager) -> Element {
        poly!(<yoyo::View::<States> variant={ States::Initial } style={ self.style.clone() }
                      on_pointer_down={ self.on_pointer_down.clone() }
                        on_pointer_up={ self.on_pointer_up.clone() }>
            { manager.children() }
        </yoyo::View::<States>>)
    }
}
