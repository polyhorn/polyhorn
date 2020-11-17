use futures::channel::oneshot;
use polyhorn::{Element, Sender};
use std::time::Duration;

pub enum Automation {
    Render(Box<dyn FnOnce() -> Element + Send + Sync>),
    Snapshot {
        test_name: &'static str,
        snapshot_name: String,
    },
    OpenURL(String),
    Wait(Duration),
}

pub struct Request<T> {
    pub data: T,
    pub done: oneshot::Sender<()>,
}

/// Object that is optionally passed to each Polyhorn test and that can be used
/// to automate rendering, snapshoting, simulating UI events, etc.
pub struct Automator {
    name: &'static str,
    sender: Sender<Request<Automation>>,
}

impl Automator {
    /// Returns a new automator with the given name that sends its requests
    /// over a mpsc channel using the given sender.
    pub fn new(name: &'static str, sender: Sender<Request<Automation>>) -> Automator {
        Automator { name, sender }
    }

    /// Renders the result of the given closure.
    pub async fn render<F>(&mut self, render: F)
    where
        F: FnOnce() -> Element + Send + Sync + 'static,
    {
        self.request(Automation::Render(Box::new(render))).await
    }

    /// Takes a new screenshot and turns it into a snapshot with the given name.
    pub async fn snapshot(&mut self, name: &str) {
        self.request(Automation::Snapshot {
            test_name: self.name,
            snapshot_name: name.to_owned(),
        })
        .await
    }

    /// Opens an URL on the active device.
    pub async fn open_url(&mut self, url: &str) {
        self.request(Automation::OpenURL(url.to_owned())).await;
    }

    /// Waits for the given duration before continuing.
    pub async fn wait(&mut self, duration: Duration) {
        self.request(Automation::Wait(duration)).await;
    }

    async fn request(&mut self, automation: Automation) {
        let (sender, receiver) = oneshot::channel();

        self.sender
            .send(Request {
                data: automation,
                done: sender,
            })
            .await
            .unwrap();

        receiver.await.unwrap();
    }
}
