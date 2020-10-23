use ansi_term::Colour::Red;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::core::Executioner;
use crate::ios::simctl;
use crate::ios::tasks::{self, IOSContext, IOSTask};
use crate::Config;

/// iOS specific implementation of the `polyhorn run` command.
pub fn run(config: Config) {
    let mut simctl = simctl::Simctl::new();
    let manager = simctl.list().unwrap();
    let device = select_device(&manager);

    let result = Executioner::execute(
        &[
            IOSTask::InstallDependencies(tasks::InstallDependencies {
                dependencies: vec![
                    tasks::Dependency::cli(
                        "resvg",
                        &["resvg", "-V"],
                        &["cargo", "install", "resvg"],
                    ),
                    tasks::Dependency::cli(
                        "xcodegen",
                        &["xcodegen", "--version"],
                        &["brew", "install", "xcodegen"],
                    ),
                ],
            }),
            IOSTask::InstallTarget(tasks::InstallTarget("x86_64-apple-ios")),
            IOSTask::BuildRuntimeLibrary(tasks::BuildRuntimeLibrary {
                target: "x86_64-apple-ios",
                profile: "debug",
            }),
            IOSTask::CreateUniversalBinary(tasks::CreateUniversalBinary),
            IOSTask::GenerateXcassets(tasks::GenerateXcassets),
            IOSTask::GenerateXcodeproj(tasks::GenerateXcodeproj),
            IOSTask::BuildXcodeproj(tasks::BuildXcodeproj {
                scheme: config.spec.app.name.clone() + "_iOS",
                configuration: "Debug".to_owned(),
                destination_platform: "iOS Simulator".to_owned(),
                destination_name: device.name().to_owned(),
            }),
            IOSTask::BootIOSSimulator(tasks::BootIOSSimulator {
                name: device.name().to_owned(),
                identifier: device.identifier().to_owned(),
            }),
            IOSTask::OpenIOSSimulator(tasks::OpenIOSSimulator),
            IOSTask::InstallOnIOSSimulator(tasks::InstallOnIOSSimulator {
                name: device.name().to_owned(),
                identifier: device.identifier().to_owned(),
                configuration: "Debug".to_owned(),
            }),
            IOSTask::RunOnIOSSimulator(tasks::RunOnIOSSimulator {
                name: device.name().to_owned(),
                identifier: device.identifier().to_owned(),
            }),
        ],
        IOSContext {
            config,
            products: Default::default(),
            universal_binary_path: None,
        },
    );

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}: {:?}", Red.bold().paint("error"), error);
            std::process::exit(1);
        }
    }
}

pub fn select_device<'a>(manager: &'a simctl::DeviceManager) -> &'a simctl::Device<'a> {
    let mut devices = manager
        .query()
        .into_iter()
        .filter(|device| device.name().starts_with("iPhone") || device.name().starts_with("iPad"))
        .collect::<Vec<_>>();

    let selections = devices
        .iter()
        .map(|device| {
            device.name().to_owned()
                + match device.state() {
                    simctl::DeviceState::Booted => " [booted]",
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

    devices.remove(selection)
}
