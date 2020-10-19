use serde::{Deserialize, Serialize};
use std::future::Future;

use super::{Context, EffectLink, Key, Platform, Reference, State};

#[macro_export]
macro_rules! use_id {
    () => {{
        struct ID;
        std::rc::Rc::new(std::any::TypeId::of::<ID>())
    }};
}

pub trait UseState {
    fn use_state<S>(&mut self, key: Key, initial_value: S) -> State<S>
    where
        S: Serialize + for<'b> Deserialize<'b> + 'static;
}

#[macro_export]
macro_rules! use_state {
    ($manager:expr) => {
        use_state!($manager, Default::default())
    };
    ($manager:expr, $initial_value:expr) => {
        $crate::UseState::use_state::<_>($manager, $crate::use_id!().into(), $initial_value)
    };
}

pub trait UseReference {
    fn use_reference<R, I>(&mut self, key: Key, initializer: I) -> Reference<R>
    where
        R: 'static,
        I: FnOnce() -> R;
}

#[macro_export]
macro_rules! use_reference {
    ($manager:expr, $value:expr) => {
        $crate::UseReference::use_reference($manager, $crate::use_id!().into(), || $value)
    };
}

pub trait UseEffect<P>
where
    P: Platform + ?Sized,
{
    fn use_effect<F>(&mut self, key: Key, conditions: Option<Key>, effect: F)
    where
        F: FnOnce(&EffectLink<P>) + 'static;
}

#[macro_export]
macro_rules! use_effect {
    ($manager:expr, $effect:expr) => {
        $crate::UseEffect::use_effect($manager, $crate::use_id!().into(), None, $effect)
    };
}

pub trait UseLayoutEffect<P>
where
    P: Platform + ?Sized,
{
    fn use_layout_effect<F>(&mut self, key: Key, conditions: Option<Key>, effect: F)
    where
        F: FnOnce(&EffectLink<P>, &mut P::CommandBuffer) + 'static;
}

#[macro_export]
macro_rules! use_layout_effect {
    ($manager:expr, $effect:expr) => {
        $crate::UseLayoutEffect::use_layout_effect(
            $manager,
            $crate::use_id!().into(),
            None,
            $effect,
        )
    };
}

pub trait UseAsync {
    fn use_async<F>(&mut self, key: Key, task: F)
    where
        F: Future<Output = ()> + 'static;
}

#[macro_export]
macro_rules! use_async {
    ($manager:expr, $future:expr) => {
        $crate::UseAsync::use_async($manager, $crate::use_id!().into(), $future)
    };
}

pub trait UseContext {
    fn use_context<T>(&mut self) -> Option<Context<T>>
    where
        T: 'static;
}

#[macro_export]
macro_rules! use_context {
    ($manager:expr) => {
        $crate::UseContext::use_context($manager)
    };
}
