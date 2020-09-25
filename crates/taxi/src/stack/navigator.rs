use polyhorn::*;
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
    pub header_style: Style,
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
            header_style: Style {
                background_color: Color::from_rgba(247, 247, 247, 0.8),
                ..Default::default()
            },
            header_tint_color: Color::from_hex(0x007AFF),
            header_title_style: TextStyle {
                color: Some(Color::from_hex(0x000000)),
                font: Some(Font::bold_system_font(17.0)),
                ..Default::default()
            },
        }
    }
}

impl<T> Component for Navigator<T>
where
    T: Screen,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let selected: Reference<Vec<T>> = use_reference!(manager);
        let marker = use_state!(manager, ());

        let on_navigate = with!((selected, marker), |screen: T| {
            selected.clone().apply(|selected| selected.push(screen));

            marker.replace(());
        });

        let on_pop = with!((selected, marker), |_| {
            selected.clone().apply(|selected| selected.pop());

            marker.replace(());
        });

        let on_back_press = with!((selected, marker), |_| {
            selected.clone().apply(move |selected: &mut Vec<T>| {
                if selected.len() > 1 {
                    selected.pop();
                }
            });

            marker.replace(());
        });

        if selected.is_none() {
            selected.replace(vec![T::default()]);
        }

        let selected = selected.to_owned().unwrap();

        let depth = selected.len() - 1;

        let last = selected.last().cloned().unwrap();

        let containers: Vec<_> = selected.into_iter().enumerate().map(|(i, screen)| {
            poly!(<ScreenContainer::<T> key={ i } depth={ depth - i } screen=screen />)
        }).collect();

        poly!(<View style={ style! {
            flex_grow: 1.0;
        } } ...>
            <NavigationProvider on_navigate=on_navigate on_pop=on_pop>
                <NavigationBar style={ self.header_style.clone() }>
                    <yoyo::AnimatePresence::<ItemContext> initial=false ...>
                        <NavigationItem key={ ("Item", depth) }
                                 tint_color={ self.header_tint_color.clone() }
                                       left={ Element::empty() }
                                      title={ last.render_header_title() }
                                      right={ last.render_header_right() }
                                title_style={ self.header_title_style.clone() }
                              on_back_press={ if depth > 0 {
                            on_back_press.into()
                        } else {
                            EventListener::none()
                        } } />
                    </yoyo::AnimatePresence::<ItemContext>>
                </NavigationBar>
                <View style={ style! {
                    flex_grow: 1.0;
                } } ...>
                    <yoyo::AnimatePresence::<ContainerContext> initial=false ...>
                        { containers }
                    </yoyo::AnimatePresence::<ContainerContext>>
                </View>
            </NavigationProvider>
        </View>)
    }
}
