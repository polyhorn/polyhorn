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

        if selected.is_none() {
            selected.replace(vec![T::default()]);
        }

        let containers: Vec<_> = selected
            .to_owned()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, screen)| {
                let element = Element::new(Key::new(i), screen.into(), Element::empty());
                match i {
                    0 => element,
                    _ => poly!(
                        <ModalContainer key={ i } on_dismiss={ with!((selected, marker), |_| {
                            selected.clone().apply(|selected| selected.pop());
                            marker.replace(());
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
