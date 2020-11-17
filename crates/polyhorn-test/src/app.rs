use futures::channel::oneshot::Sender;
use polyhorn::prelude::*;
use polyhorn::{Receiver, Reference};

use super::automator::{Automation, Automator, Request};
use super::channel::Handler;
use super::client::Client;

/// Component that is responsible for rendering Polyhorn tests.
#[derive(Default)]
pub struct App {}

impl Component for App {
    fn render(&self, manager: &mut Manager) -> Element {
        let element: Reference<Option<Element>> = use_reference!(manager, None);
        let on_render: Reference<Option<Sender<()>>> = use_reference!(manager, None);

        let sender = use_channel!(manager, {
            let element = element.weak(manager);
            let on_render = on_render.weak(manager);

            let client = Client::new(std::env::var("POLYHORN_TEST_FEEDBACK_URL").unwrap());
            let handler = Handler::new(client, element, on_render);

            |mut receiver: Receiver<Request<Automation>>| async move {
                while let Some(message) = receiver.next().await {
                    handler.on_request(message).await;
                }
            }
        });

        use_async!(manager, async move {
            let hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
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
