use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::{UIStatusBar, UIStatusBarStyle, UIWindow};

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusBarStyle {
    DarkContent,
    LightContent,
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        StatusBarStyle::DarkContent
    }
}

pub struct StatusBar {
    pub style: StatusBarStyle,
}

impl Component for StatusBar {
    fn render(&self, manager: &mut Manager) -> Element {
        let style = self.style;

        use_effect!(manager, move |buffer| {
            buffer.mutate(&[], move |_| {
                let window = UIWindow::key();
                let mut status_bar = UIStatusBar::new(&window);
                status_bar.set_style(match style {
                    StatusBarStyle::LightContent => UIStatusBarStyle::LightContent,
                    StatusBarStyle::DarkContent => UIStatusBarStyle::DarkContent,
                });
            });
        });

        manager.children()
    }
}
