use futures::channel::{mpsc, oneshot};
use futures::select;
use futures::stream::FuturesUnordered;
use futures::FutureExt;
use futures::StreamExt;
use std::cell::RefCell;
use std::future::Future;
use std::mem::take;
use std::pin::Pin;
use std::rc::Rc;

pub enum Message {
    Refresh,
    Terminate,
}

#[derive(Default)]
pub struct BusState {
    additions: Vec<Pin<Box<dyn Future<Output = Option<Message>>>>>,
}

/// Concrete implementation of the event bus that is used internally.
pub struct Bus {
    state: Rc<RefCell<BusState>>,
    tx: mpsc::Sender<Message>,
}

pub struct Handler {
    state: Rc<RefCell<BusState>>,
    rx: mpsc::Receiver<Message>,
}

pub struct Token {
    tx: Option<oneshot::Sender<()>>,
}

impl Drop for Token {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            // Note: we ignore the result. The result will be an error if the
            // future has already finished, in which case the receiver is
            // dropped.
            let _ = tx.send(());
        }
    }
}

impl Bus {
    /// Returns a new bus and its handler. The handler should be send to the
    /// dedicated Polyhorn thread while the reference-counted bus itself can be
    /// shared among all threads.
    pub fn new() -> (Bus, Handler) {
        let state = Rc::new(RefCell::new(BusState::default()));

        let (tx, rx) = mpsc::channel::<Message>(1024);

        (
            Bus {
                state: state.clone(),
                tx,
            },
            Handler { state, rx },
        )
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let _ = self.tx.try_send(Message::Terminate);
    }
}

impl Handler {
    pub async fn main(mut self) {
        let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Option<Message>>>>>::new();

        loop {
            select! {
                message = self.rx.next() => {
                    match message {
                        Some(Message::Refresh) => {
                            let pending = take(&mut self.state.borrow_mut().additions);
                            tasks.extend(pending);
                        }
                        Some(Message::Terminate) => break,
                        None => {},
                    }
                },
                _ = tasks.next() => {},
            };
        }
    }
}

impl polyhorn_core::Bus for Bus {
    fn queue<F>(&self, task: F) -> polyhorn_core::Disposable
    where
        F: Future<Output = ()> + 'static,
    {
        // This will always succeed since the handler does not hold a persistent
        // reference to the pending state.
        let mut pending = self.state.borrow_mut();

        let (tx, rx) = oneshot::channel();

        pending.additions.push(Box::pin(async move {
            let mut rx = rx.fuse();
            let mut task = Box::pin(task).fuse();

            select! {
                _ = rx => {},
                _ = task => {},
            };

            Some(Message::Refresh)
        }));

        self.tx.clone().try_send(Message::Refresh).unwrap();

        polyhorn_core::Disposable::new(Token { tx: Some(tx) })
    }

    fn queue_retain<F>(&self, task: F)
    where
        F: Future<Output = ()> + 'static,
    {
        // This will always succeed since the handler does not hold a persistent
        // reference to the pending state.
        let mut pending = self.state.borrow_mut();

        pending.additions.push(Box::pin(async move {
            task.await;

            Some(Message::Refresh)
        }));

        self.tx.clone().try_send(Message::Refresh).unwrap();
    }
}
