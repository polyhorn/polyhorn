use futures::channel::oneshot::{channel, Sender};
use polyhorn::prelude::*;
use polyhorn::WeakReference;

use super::automator::{Automation, Request};
use super::client::{Client, Message};

pub struct Handler {
    client: Client,
    element: WeakReference<Option<Element>>,
    on_render: WeakReference<Option<Sender<()>>>,
}

impl Handler {
    pub fn new(
        client: Client,
        element: WeakReference<Option<Element>>,
        on_render: WeakReference<Option<Sender<()>>>,
    ) -> Handler {
        Handler {
            client,
            element,
            on_render,
        }
    }

    pub async fn on_request(&self, request: Request<Automation>) {
        match request.data {
            Automation::Render(render) => {
                let (sender, receiver) = channel::<()>();

                self.element.replace(Some(render()));
                self.on_render.replace(Some(sender));
                self.element.queue_rerender();

                receiver.await.unwrap();
            }
            Automation::OpenURL(url) => {
                self.client.send(&Message::OpenURL(url)).await;
            }
            Automation::Snapshot {
                test_name,
                snapshot_name,
            } => {
                self.client
                    .send(&Message::Snapshot {
                        test_name,
                        snapshot_name,
                    })
                    .await;
            }
            Automation::Wait(duration) => {
                tokio::time::delay_for(duration).await;
            }
        }

        request.done.send(()).unwrap();
    }
}
