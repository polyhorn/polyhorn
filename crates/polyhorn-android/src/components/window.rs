use polyhorn_core::{CommandBuffer, Key};

use crate::prelude::*;
use crate::raw::{Builtin, Convert};

impl Component for Window {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager, None);
        let style = self.style;

        use_layout_effect!(manager, move |link, buffer| {
            if let Some(view) = reference.apply(link, |&mut id| id) {
                buffer.mutate(&[view], move |views, environment| {
                    if let Some(view) = views[0].downcast_mut::<polyhorn_android_sys::View>() {
                        view.set_background_color(
                            environment.env(),
                            style.background_color.convert(environment.env()),
                        );
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
