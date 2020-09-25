use super::navigator::NavigationProvider;
use polyhorn::*;
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
        let selected = use_reference!(manager);
        let marker: State<()> = use_state!(manager);

        if selected.is_none() {
            selected.replace(T::default());
        }

        let on_navigate = with!((selected, marker), |screen| {
            selected.replace(screen);
            marker.replace(());
        });

        let on_pop = with!((selected, marker), |_| {
            // Unimplemented.
        });

        let mut items = vec![];

        for (i, screen) in T::all().into_iter().enumerate() {
            let is_selected = selected.to_owned().unwrap().selected_index() == i;
            let title = screen.options().title.clone();
            let on_press = with!((selected, screen, marker), |_| {
                selected.replace(screen.clone());
                marker.replace(());
            });
            items.push(poly!(
                <TabBarItem key=i selected=is_selected title=title
                       on_press=on_press ... />
            ));
        }

        poly!(<View style={ style! {
            flex_grow: 1.0;
        } } ...>
            <View style={ style! {
                flex_grow: 1.0;
            } } ...>
                <NavigationProvider on_navigate=on_navigate on_pop=on_pop>
                    { Element::new(Key::new(()), selected.to_owned().unwrap().into(), Element::fragment(Key::new(()), vec![])) }
                </NavigationProvider>
            </View>
            <TabBar>
                { items }
            </TabBar>
        </View>)
    }
}
