use polyhorn::*;
use yoyo::{components::DynPresence, use_presence, Presence};

use super::ModalContext;

pub struct ModalContainer {
    pub on_dismiss: EventListener<()>,
}

impl Component for ModalContainer {
    fn render(&self, manager: &mut Manager) -> Element {
        let presence: Presence<ModalContext> = use_presence!(manager);
        let is_present = presence.is_present();
        let safe_to_remove = presence.safe_to_remove();

        let dismissed = use_reference!(manager);

        if !is_present && dismissed.is_some() {
            safe_to_remove.invoke();
        }

        let on_dismiss = self.on_dismiss.clone();
        let on_dismiss = move |event| {
            dismissed.replace(());

            if is_present {
                on_dismiss.call(event);
            } else {
                safe_to_remove.invoke();
            }
        };

        poly!(
            <Modal visible={ presence.is_present() } on_dismiss=on_dismiss>
                { manager.children() }
            </Modal>
        )
    }
}
