use polyhorn::prelude::*;
use polyhorn::Reference;
use polyhorn_ui::color::Color;
use polyhorn_ui::events::EventListener;
use polyhorn_ui::styles::{TextStyle, ViewStyle};
use std::marker::PhantomData;

use super::{
    ContainerContext, ItemContext, NavigationBar, NavigationItem, Screen, ScreenContainer,
};
use crate::navigator::NavigationProvider;

pub struct Navigator<T>
where
    T: Screen,
{
    pub marker: PhantomData<T>,
    pub header_style: ViewStyle,
    pub header_tint_color: Color,
    pub header_title_style: TextStyle,
}

impl<T> Default for Navigator<T>
where
    T: Screen,
{
    fn default() -> Self {
        Navigator {
            marker: Default::default(),
            header_style: ViewStyle {
                background_color: Color::rgba(247, 247, 247, 0.8),
                ..Default::default()
            },
            header_tint_color: Color::hex(0x007AFF),
            header_title_style: style! {
                color: black;
                font-size: 17px;
                font-weight: bold;
            },
        }
    }
}

impl<T> Component for Navigator<T>
where
    T: Screen,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let selected: Reference<Vec<T>> = use_reference!(manager, vec![T::default()]);
        let marker = use_state!(manager, ());

        let on_navigate = manager.bind(move |link, screen: T| {
            selected.apply(link, |selected| selected.push(screen));
            marker.replace(link, ());
        });

        let on_pop = manager.bind(move |link, _| {
            selected.apply(link, |selected| selected.pop());
            marker.replace(link, ());
        });

        let on_back_press = manager.bind(move |link, _| {
            selected.apply(link, move |selected: &mut Vec<T>| {
                if selected.len() > 1 {
                    selected.pop();
                }
            });

            marker.replace(link, ());
        });

        let selected = selected.apply(manager, |selected| selected.to_owned());

        let depth = selected.len() - 1;

        let last = selected.last().cloned().unwrap();

        let containers: Vec<_> = selected.into_iter().enumerate().map(|(i, screen)| {
            poly!(<ScreenContainer::<T> key={ i } depth={ depth - i } screen=screen default=false />)
        }).collect();

        poly!(<View style={ style! {
            flex-grow: 1.0;
        } }>
            <NavigationProvider on_navigate=on_navigate on_pop=on_pop>
                <NavigationBar style={ self.header_style.clone() }>
                    <yoyo::AnimatePresence::<ItemContext> initial=false>
                        <NavigationItem key={ ("Item", depth) }
                                 tint_color={ self.header_tint_color }
                                       left={ Element::empty() }
                                      title={ last.render_header_title() }
                                      right={ last.render_header_right() }
                                title_style={ self.header_title_style }
                              on_back_press={ if depth > 0 {
                            on_back_press.into()
                        } else {
                            EventListener::default()
                        } } />
                    </yoyo::AnimatePresence::<ItemContext>>
                </NavigationBar>
                <View style={ style! {
                    flex-grow: 1.0;
                } }>
                    <yoyo::AnimatePresence::<ContainerContext> initial=false>
                        { containers }
                    </yoyo::AnimatePresence::<ContainerContext>>
                </View>
            </NavigationProvider>
        </View>)
    }
}
