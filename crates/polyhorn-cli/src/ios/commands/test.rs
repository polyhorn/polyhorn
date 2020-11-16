use ansi_term::Colour::Red;
use simctl::Simctl;

use super::select_device;
use crate::core::Executioner;
use crate::ios::tasks::{self, IOSContext, IOSTask};
use crate::Config;

/// iOS specific implementation of the `polyhorn test` command.
pub fn test(config: Config) {
    let device = select_device(Simctl::new().list().unwrap().devices());

    let (addr, _server) = crate::test::serve(device.clone());

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
            IOSTask::BuildRuntimeLibraryV2(tasks::BuildRuntimeLibraryV2 {
                cfg: "test",
                target: "x86_64-apple-ios",
                profile: "dev",
                flags: &[],
            }),
            IOSTask::CreateUniversalBinary(tasks::CreateUniversalBinary),
            IOSTask::GenerateXcassets(tasks::GenerateXcassets),
            IOSTask::GenerateXcodeproj(tasks::GenerateXcodeproj),
            IOSTask::BuildXcodeproj(tasks::BuildXcodeproj {
                scheme: config.spec.app.name.clone() + "_iOS",
                configuration: "Debug".to_owned(),
                destination_platform: "iOS Simulator".to_owned(),
                destination_name: device.name.clone(),
            }),
            IOSTask::BootIOSSimulator(tasks::BootIOSSimulator {
                device: device.clone(),
            }),
            IOSTask::InstallOnIOSSimulator(tasks::InstallOnIOSSimulator {
                device: device.clone(),
                configuration: "Debug".to_owned(),
            }),
            IOSTask::RunOnIOSSimulator(tasks::RunOnIOSSimulator {
                device: device.clone(),
                env: vec![(
                    "POLYHORN_TEST_FEEDBACK_URL".to_owned(),
                    format!("http://{}/polyhorn/tests/test", addr),
                )],
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
