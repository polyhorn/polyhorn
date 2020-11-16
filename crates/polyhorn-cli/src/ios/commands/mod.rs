//! iOS-specific implementations of Polyhorn CLI commands.

mod run;
mod test;

pub use run::run;
pub use test::test;

use dialoguer::{theme::ColorfulTheme, Select};
use simctl::DeviceQuery;

fn select_device(devices: &[simctl::Device]) -> simctl::Device {
    let mut devices = devices
        .iter()
        .available()
        .filter(|device| device.name.starts_with("iPhone") || device.name.starts_with("iPad"))
        .collect::<Vec<_>>();

    let selections = devices
        .iter()
        .map(|device| {
            device.name.to_owned()
                + match device.state {
                    simctl::list::DeviceState::Booted => " [booted]",
                    _ => "",
                }
        })
        .collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your device")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    devices.remove(selection).clone()
}
