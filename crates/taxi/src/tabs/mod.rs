use super::navigator::NavigationProvider;
use polyhorn::prelude::*;
use polyhorn::{Key, State};
use std::marker::PhantomData;

mod bar;

pub use bar::{TabBar, TabBarItem};

pub trait Routes: Default + Clone + Component + Screen + 'static {
    fn all() -> Vec<Self>;
    fn selected_index(&self) -> usize;
}

pub struct ScreenOptions {
    pub title: String,
}

pub trait Screen {
    fn options(&self) -> ScreenOptions;
}

#[derive(Default)]
pub struct Navigator<T>
where
    T: Routes,
{
    pub marker: PhantomData<T>,
}

impl<T> Component for Navigator<T>
where
    T: Routes,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let selected = use_reference!(manager, T::default());
        let marker: State<()> = use_state!(manager);

        let on_navigate = manager.bind(move |link, screen| {
            selected.replace(link, screen);
            marker.replace(link, ());
        });

        let on_pop = |_| unreachable!("Tab navigators should not be popped.");

        let mut items = vec![];

        for (i, screen) in T::all().into_iter().enumerate() {
            let is_selected = selected.apply(manager, |selected| selected.selected_index()) == i;
            let title = screen.options().title.clone();
            let on_press = manager.bind(move |link, _| {
                selected.replace(link, screen.clone());
                marker.replace(link, ());
            });
            items.push(poly!(
                <TabBarItem key=i selected=is_selected title=title
                       on_press=on_press />
            ));
        }

        poly!(<View style={ style! {
            flex-grow: 1.0;
        } }>
            <View style={ style! {
                flex-grow: 1.0;
            } }>
                <NavigationProvider on_navigate=on_navigate on_pop=on_pop>
                    { Element::new(Key::new(()), selected.apply(manager, |selected| selected.clone()).into(), Element::fragment(Key::new(()), vec![])) }
                </NavigationProvider>
            </View>
            <TabBar>
                { items }
            </TabBar>
        </View>)
    }
}
