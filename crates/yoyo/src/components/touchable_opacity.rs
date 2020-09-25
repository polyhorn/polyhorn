use crate as yoyo;
use polyhorn::*;

use super::View;

#[derive(Default)]
pub struct TouchableOpacity {
    pub style: Style,
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
        opacity_transition: ease_in_out;
    }
});

impl Component for TouchableOpacity {
    fn render(&self, manager: &mut Manager) -> Element {
        poly!(<View::<States> variant={ States::Initial } style={ self.style.clone() }
                      on_pointer_down={ self.on_pointer_down.clone() }
                        on_pointer_up={ self.on_pointer_up.clone() } ...>
            { manager.children() }
        </View::<States>>)
    }
}
