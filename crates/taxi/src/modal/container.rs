use polyhorn::prelude::*;
use polyhorn_ui::events::EventListener;
use yoyo::{components::DynPresence, use_presence, Presence};

use super::ModalContext;

#[derive(Default)]
pub struct ModalContainer {
    pub on_dismiss: EventListener<()>,
}

impl Component for ModalContainer {
    fn render(&self, manager: &mut Manager) -> Element {
        let presence: Presence<ModalContext> = use_presence!(manager);
        let is_present = presence.is_present();
        let safe_to_remove = presence.safe_to_remove();

        let dismissed = use_reference!(manager, false);

        if !is_present && dismissed.apply(manager, |&mut value| value) {
            safe_to_remove.invoke();
        }

        let dismissed = dismissed.weak(manager);

        let on_dismiss = self.on_dismiss.clone();
        let on_dismiss = move |event| {
            dismissed.replace(true);

            if is_present {
                on_dismiss.emit(event);
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
