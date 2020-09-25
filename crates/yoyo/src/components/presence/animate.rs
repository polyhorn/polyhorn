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
        let mut memory = use_reference!(manager);
        let marker = use_state!(manager, ());
        let is_animated = use_reference!(manager);

        if self.initial {
            is_animated.replace(());
        }

        if memory.is_none() {
            let mut initial = Memory::new();

            for child in manager.children().to_vec() {
                initial.insert(child.clone());
            }

            memory.replace(initial);
        }

        let present: HashSet<_> = manager
            .children()
            .to_vec()
            .into_iter()
            .map(|child| child.key().clone())
            .collect();

        let present: HashSet<_> = memory
            .apply(|memory| {
                let mut ids = vec![];

                for element in manager.children().to_vec().into_iter().cloned() {
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
            })
            .unwrap_or_default();

        let snapshot = memory.to_owned().unwrap();

        use_effect!(
            manager,
            with!((is_animated), |_| {
                is_animated.replace(());
            })
        );

        Element::fragment(
            Key::new(()),
            snapshot
                .elements_by_id()
                .cloned()
                .map(|(id, element)| {
                    let presence = PresenceContext {
                        custom: self.custom.clone(),
                        is_animated: is_animated.is_some(),
                        is_present: present.contains(&id),
                        safe_to_remove: SafeToRemove {
                            id,
                            marker: marker.clone(),
                            memory: memory.clone(),
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
