use clap::Clap;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::canonicalize;
use std::sync::mpsc::channel;
use std::time::Duration;

use super::Platform;
use crate::ios::simctl;

/// Watches your project for changes and automatically reloads the running app on
/// a device or simulator.
#[derive(Clap)]
pub struct Watch {
    #[clap(subcommand)]
    platform: Platform,
}

impl Watch {
    pub fn main(&self) {
        let _ = self.platform;

        let mut simctl = simctl::Simctl::new();
        let manager = simctl.list().unwrap();
        let _device = super::run::select_device(&manager);

        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_millis(1000)).unwrap();

        watcher.watch(".", RecursiveMode::Recursive).unwrap();

        let target = canonicalize("target/").unwrap();

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(path) if !path.starts_with(&target) => {
                        super::run::build_cargo();

                        unimplemented!();
                    }
                    _ => {}
                },
                Err(_) => break,
            }
        }
    }
}
