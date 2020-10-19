use polyhorn::prelude::*;
use polyhorn::Key;
use std::collections::HashSet;

use super::{Memory, PresenceContext, SafeToRemove};

#[derive(Default)]
pub struct AnimatePresence<T>
where
    T: Clone + 'static,
{
    pub custom: T,
    pub initial: bool,
}

impl<T> Component for AnimatePresence<T>
where
    T: Clone + 'static,
{
    fn render(&self, manager: &mut Manager) -> Element {
        let children = manager
            .children()
            .to_vec()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let memory = use_reference!(manager, {
            let mut initial = Memory::new();

            for child in children {
                initial.insert(child);
            }

            initial
        });

        let is_animated = use_reference!(manager, false);

        if self.initial {
            is_animated.replace(manager, true);
        }

        let present: HashSet<_> = manager
            .children()
            .to_vec()
            .into_iter()
            .map(|child| child.key().clone())
            .collect();

        let children = manager
            .children()
            .to_vec()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let present: HashSet<_> = memory.apply(manager, |memory| {
            let mut ids = vec![];

            for element in children {
                if let Some((id, existing)) = memory.lookup(&element.key()) {
                    *existing = element;
                    ids.push(id);
                } else {
                    ids.push(memory.insert(element));
                }
            }

            for key in memory.keys().cloned().collect::<Vec<_>>() {
                if present.contains(&key) {
                    continue;
                }

                memory.forget(&key);
            }

            ids.into_iter().collect()
        });

        let snapshot = memory.apply(manager, |memory| memory.to_owned());

        use_effect!(manager, move |link| {
            is_animated.replace(link, true);
        });

        Element::fragment(
            Key::new(()),
            snapshot
                .elements_by_id()
                .cloned()
                .map(|(id, element)| {
                    let presence = PresenceContext {
                        custom: self.custom.clone(),
                        is_animated: is_animated.apply(manager, |&mut value| value),
                        is_present: present.contains(&id),
                        safe_to_remove: SafeToRemove {
                            id,
                            memory: memory.weak(manager),
                        },
                    };

                    poly!(
                        <ContextProvider key={ id } value=presence>
                            { element }
                        </ContextProvider>
                    )
                })
                .collect(),
        )
    }
}
