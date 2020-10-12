use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::polykit::{PLYStatusBar, PLYWindow};
use polyhorn_ios_sys::uikit::UIStatusBarStyle;

use crate::prelude::*;

impl Component for StatusBar {
    fn render(&self, manager: &mut Manager) -> Element {
        let style = self.style;

        use_effect!(manager, move |_, buffer| {
            buffer.mutate(&[], move |_| {
                let window = PLYWindow::key_window();
                let mut status_bar = PLYStatusBar::new(&window);
                status_bar.set_style(match style {
                    StatusBarStyle::LightContent => UIStatusBarStyle::LightContent,
                    StatusBarStyle::DarkContent => UIStatusBarStyle::DarkContent,
                });
            });
        });

        manager.children()
    }
}
