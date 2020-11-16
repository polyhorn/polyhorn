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

pub struct Automator {
    name: &'static str,
    sender: Sender<Request<Automation>>,
}

impl Automator {
    pub fn new(name: &'static str, sender: Sender<Request<Automation>>) -> Automator {
        Automator { name, sender }
    }

    pub async fn render<F>(&mut self, render: F)
    where
        F: FnOnce() -> Element + Send + Sync + 'static,
    {
        self.request(Automation::Render(Box::new(render))).await
    }

    pub async fn snapshot(&mut self, name: &str) {
        self.request(Automation::Snapshot {
            test_name: self.name,
            snapshot_name: name.to_owned(),
        })
        .await
    }

    pub async fn open_url(&mut self, url: &str) {
        self.request(Automation::OpenURL(url.to_owned())).await;
    }

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
