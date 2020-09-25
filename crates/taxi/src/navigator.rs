use polyhorn::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Navigator<T>(Context<NavigationContext<T>>);

impl<T> Navigator<T> {
    pub fn navigate(&self, screen: T) {
        self.0.upgrade().unwrap().on_navigate.call(screen);
    }

    pub fn pop(&self) {
        self.0.upgrade().unwrap().on_pop.call(());
    }
}

pub struct NavigationContext<T> {
    on_navigate: EventListener<T>,
    on_pop: EventListener<()>,
}

pub struct NavigationProvider<T> {
    pub on_navigate: EventListener<T>,
    pub on_pop: EventListener<()>,
}

impl<T> Component for NavigationProvider<T>
where
    T: 'static,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let value = Rc::new(NavigationContext {
            on_navigate: self.on_navigate.clone(),
            on_pop: self.on_pop.clone(),
        });
        poly!(<ContextProvider::<NavigationContext::<T>> value=value>
            { manager.children() }
        </ContextProvider::<NavigationContext::<T>>>)
    }
}

pub trait UseNavigator<T> {
    fn use_navigator(&mut self) -> Navigator<T>;
}

impl<T, M> UseNavigator<T> for M
where
    T: 'static,
    M: UseContext,
{
    fn use_navigator(&mut self) -> Navigator<T> {
        Navigator(use_context!(self).unwrap())
    }
}

#[macro_export]
macro_rules! use_navigator {
    ($manager:expr) => {
        $crate::UseNavigator::use_navigator($manager)
    };
}
