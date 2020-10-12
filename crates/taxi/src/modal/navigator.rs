use polyhorn::prelude::*;
use polyhorn::{Key, Reference};
use std::marker::PhantomData;
use yoyo::AnimatePresence;

use super::{ModalContainer, ModalContext, Screen};
use crate::navigator::NavigationProvider;

pub struct Navigator<T>
where
    T: Screen,
{
    pub marker: PhantomData<T>,
}

impl<T> Default for Navigator<T>
where
    T: Screen,
{
    fn default() -> Self {
        Navigator {
            marker: Default::default(),
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

        let containers: Vec<_> = selected
            .apply(manager, |selected| selected.clone())
            .into_iter()
            .enumerate()
            .map(|(i, screen)| {
                let element = Element::new(Key::new(i), screen.into(), Element::empty());
                match i {
                    0 => element,
                    _ => poly!(
                        <ModalContainer key={ i } on_dismiss={ manager.bind(move |link, _| {
                            selected.apply(link, |selected| selected.pop());
                            marker.replace(link, ());
                        }) }>
                            { element }
                        </ModalContainer>
                    ),
                }
            })
            .collect();

        poly!(<View style={ style! {
            flex-grow: 1.0;
        } } ...>
            <NavigationProvider on_navigate=on_navigate on_pop=on_pop>
                <AnimatePresence::<ModalContext> ...>
                    { containers }
                </AnimatePresence::<ModalContext>>
            </NavigationProvider>
        </View>)
    }
}
