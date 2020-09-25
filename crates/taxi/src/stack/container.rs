use super::{ContainerContext, Screen};
use polyhorn::*;
use yoyo::{use_presence, Presence};

yoyo::yoyo!(States, {
    opacity: 1.0;
    opacity_transition: ease_in_out;

    // These are the parameters that iOS uses.
    transform_translation_x_transition: spring(1000, 500, 3, false, true);

    :initial {
        transform_translation_x: 375px;
    }

    :exit {
        transform_translation_x: 375px;
    }

    .foreground {
        transform_translation_x: 0px;
        opacity: 1.0;
    }

    .background {
        transform_translation_x: -187.5px;
        opacity: 0.5;

        &:from(.idle) {
            opacity_transition: delay(0.4);
        }
    }

    .idle {
        opacity: 0;

        &:from(.background) {
            opacity_transition: step;
        }
    }
});

pub struct ScreenContainer<T>
where
    T: Screen,
{
    pub screen: T,

    /// This is the depth of the screen with respect to the active screen.
    /// Specifically, the active screen has depth = 0, the background screen has
    /// depth 1. Any screen with depth > 1 should be hidden.
    pub depth: usize,
}

impl<T> Component for ScreenContainer<T>
where
    T: Screen,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let element = Element::new(
            Key::new(()),
            self.screen.clone().into(),
            Element::fragment(Key::new(()), vec![]),
        );

        let presence: Presence<ContainerContext> = use_presence!(manager);

        poly!(<yoyo::View::<States> presence={ presence.into_dyn() } variant={ if self.depth == 0 {
            States::Foreground
        } else if self.depth == 1 {
            States::Background
        } else {
            States::Idle
        } } style={ style! {
            position: if self.depth == 0 {
                Position::Relative
            } else {
                Position::Absolute
            };
            flex_grow: 1.0;
            visibility: if self.depth > 1 {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        } } ...>
            { element }
        </yoyo::View::<States>>)
    }
}
