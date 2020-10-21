use futures::channel::mpsc::{self, TrySendError};
use futures::{Future, StreamExt};

use crate::{Key, Link, UseAsync, UseReference};

#[macro_export]
macro_rules! use_channel {
    ($manager:expr, $task:expr) => {
        $crate::UseChannel::use_channel($manager, $crate::Key::from($crate::use_id!()), $task)
    };
}

pub struct Sender<T>(mpsc::Sender<T>)
where
    T: Send + 'static;

impl<T> Sender<T>
where
    T: Send + 'static,
{
    pub fn try_send(&mut self, message: T) -> Result<(), TrySendError<T>> {
        self.0.try_send(message)
    }
}

impl<T> Clone for Sender<T>
where
    T: Send + 'static,
{
    fn clone(&self) -> Self {
        Sender(self.0.clone())
    }
}

pub struct Receiver<T>(mpsc::Receiver<T>)
where
    T: Send + 'static;

impl<T> Receiver<T>
where
    T: Send + 'static,
{
    pub fn next(&mut self) -> impl Future<Output = Option<T>> + '_ {
        self.0.next()
    }
}

pub trait UseChannel {
    fn use_channel<T, C, F>(&mut self, key: Key, closure: C) -> Sender<T>
    where
        T: Send + 'static,
        C: FnOnce(Receiver<T>) -> F + 'static,
        F: Future<Output = ()>;
}

impl<M> UseChannel for M
where
    M: UseAsync + UseReference + Link,
{
    fn use_channel<T, C, F>(&mut self, key: Key, closure: C) -> Sender<T>
    where
        T: Send + 'static,
        C: FnOnce(Receiver<T>) -> F + 'static,
        F: Future<Output = ()>,
    {
        let mut rx = None;

        let tx = self.use_reference(key.clone(), || {
            let (new_tx, new_rx) = mpsc::channel::<T>(1024);
            rx = Some(new_rx);
            new_tx
        });

        let tx = tx.apply(self, |tx| tx.to_owned());

        self.use_async(key, async move {
            if let Some(rx) = rx.take() {
                closure(Receiver(rx)).await;
            }
        });

        Sender(tx)
    }
}
