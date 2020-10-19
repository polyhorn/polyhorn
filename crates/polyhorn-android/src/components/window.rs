use polyhorn_core::{CommandBuffer, Key};

use crate::prelude::*;
use crate::raw::Builtin;

impl Component for Window {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager, None);

        use_effect!(manager, move |link, buffer| {
            if let Some(view) = reference.apply(link, |&mut id| id) {
                buffer.mutate(&[view], |views, environment| {
                    if let Some(view) = views[0].downcast_mut::<polyhorn_android_sys::View>() {
                        view.set_background_color(environment.env(), 255, 0, 0, 1.0);
                    }
                });
            }
        });

        Element::builtin(
            Key::new(()),
            Builtin::Window,
            manager.children(),
            Some(reference.weak(manager)),
        )
    }
}
