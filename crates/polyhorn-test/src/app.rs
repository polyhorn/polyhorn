use futures::channel::oneshot::{channel, Sender};
use polyhorn::prelude::*;
use polyhorn::{Receiver, Reference};
use serde::Serialize;

use super::automator::{Automation, Request};
use super::Automator;

#[derive(Default)]
pub struct App {}

#[derive(Serialize)]
pub enum Message {
    Snapshot {
        test_name: &'static str,
        snapshot_name: String,
    },
    OpenURL(String),
}

impl Component for App {
    fn render(&self, manager: &mut Manager) -> Element {
        let element: Reference<Option<Element>> = use_reference!(manager, None);
        let on_render: Reference<Option<Sender<()>>> = use_reference!(manager, None);

        let sender = use_channel!(manager, {
            let element = element.weak(manager);
            let on_render = on_render.weak(manager);

            |mut receiver: Receiver<Request<Automation>>| async move {
                let url = std::env::var("POLYHORN_TEST_FEEDBACK_URL").unwrap();
                let client = reqwest::Client::new();

                while let Some(message) = receiver.next().await {
                    match message.data {
                        Automation::Render(render) => {
                            let (sender, receiver) = channel::<()>();

                            element.replace(Some(render()));
                            on_render.replace(Some(sender));
                            element.queue_rerender();

                            receiver.await.unwrap();
                        }
                        Automation::OpenURL(name) => {
                            client
                                .post(&url)
                                .json(&Message::OpenURL(name))
                                .send()
                                .await
                                .unwrap();
                        }
                        Automation::Snapshot {
                            test_name,
                            snapshot_name,
                        } => {
                            client
                                .post(&url)
                                .json(&Message::Snapshot {
                                    test_name,
                                    snapshot_name,
                                })
                                .send()
                                .await
                                .unwrap();
                        }
                        Automation::Wait(duration) => {
                            tokio::time::delay_for(duration).await;
                        }
                    }

                    message.done.send(()).unwrap();
                }
            }
        });

        use_async!(manager, async move {
            let hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                eprintln!("Panic!");

                hook(info);
                std::process::exit(1);
            }));

            let tests = crate::inventory::all();

            for (name, test) in tests {
                let mut automator = Automator::new(name, sender.clone());
                test.call(&mut automator).await;
            }

            std::process::exit(0);
        });

        use_effect!(manager, move |link| {
            on_render.apply(link, move |on_render| {
                if let Some(on_render) = on_render.take() {
                    on_render.send(()).unwrap();
                }
            });
        });

        poly!(<Window>
            { element.apply(manager, |element| element.as_ref().cloned()) }
        </Window>)
    }
}
