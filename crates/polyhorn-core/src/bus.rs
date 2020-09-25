use super::Disposable;
use std::future::Future;

pub trait Bus {
    fn queue<F>(&self, task: F) -> Disposable
    where
        F: Future<Output = ()> + 'static;

    fn queue_retain<F>(&self, task: F)
    where
        F: Future<Output = ()> + 'static;
}
