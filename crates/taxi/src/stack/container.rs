use polyhorn::prelude::*;
use polyhorn::Key;
use polyhorn_ui::styles::{Position, Relative, ViewStyle, Visibility};
use yoyo::{use_presence, Presence};

use super::{ContainerContext, Screen};

yoyo::yoyo!(States, {
    opacity: 1.0;
    transition-opacity: ease-in-out(0.4);

    // These are the parameters that iOS uses.
    transition-transform: spring(1000, 500, 3, false, true);

    :initial {
        transform: translateX(375px);
    }

    :exit {
        transform: translateX(375px);
    }

    .foreground {
        transform: none;
        opacity: 1.0;
    }

    .background {
        transform: translateX(-187px);
        opacity: 0.5;
    }

    .idle {
        opacity: 0;

        &:from(.background) {
            transition-opacity: step;
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

        let style = ViewStyle {
            position: if self.depth == 0 {
                Position::Relative(Relative {
                    flex_grow: 1.0,
                    ..Default::default()
                })
            } else {
                Position::Absolute(Default::default())
            },
            visibility: if self.depth > 1 {
                Visibility::Hidden
            } else {
                Visibility::Visible
            },
            ..Default::default()
        };

        poly!(<yoyo::View::<States> presence={ presence.into_dyn() } variant={ if self.depth == 0 {
            States::Foreground
        } else if self.depth == 1 {
            States::Background
        } else {
            States::Idle
        } } style=style ...>
            { element }
        </yoyo::View::<States>>)
    }
}
