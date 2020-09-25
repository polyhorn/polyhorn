use clap::Clap;
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::Platform;
use crate::ios::infoplist;
use crate::ios::simctl;
use crate::ios::xcassets::{Folder, Image, ImageSet, Info, Properties, XcAssets};
use crate::ios::xcodegen;
use crate::spec::Spec;

/// Runs the app on a device or simulator.
#[derive(Clap)]
pub struct Run {
    #[clap(subcommand)]
    platform: Platform,
}

impl Run {
    pub fn main(&self) {
        let path = build_cargo();
        build_assets();

        let mut simctl = simctl::Simctl::new();
        let manager = simctl.list().unwrap();
        let device = select_device(&manager);

        match self.platform {
            Platform::Android => unimplemented!(),
            Platform::IOS => run_ios_simulator(&device, &path),
        }
    }
}

pub fn generate_project_spec(spec: &Spec, path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create("target/ios/Sources/main.m").unwrap();
    file.write_all(&mut include_bytes!("../../ios/template/main.m.tmpl").to_owned())
        .unwrap();

    // Write the info plist.
    let infoplist = infoplist::InfoPlist {
        bundle_development_region: "$(DEVELOPMENT_LANGUAGE)",
        bundle_executable: "$(EXECUTABLE_NAME)",
        bundle_identifier: "$(PRODUCT_BUNDLE_IDENTIFIER)",
        bundle_info_dictionary_version: "6.0",
        bundle_name: "$(PRODUCT_NAME)",
        bundle_package_type: "$(PRODUCT_BUNDLE_PACKAGE_TYPE)",
        bundle_short_version_string: "1.0",
        bundle_version: "1",
        requires_iphone_os: true,
        launch_storyboard_name: "LaunchScreen",
        required_device_capabilities: &["armv7"],
        supported_interface_orientations: &["UIInterfaceOrientationPortrait"],
        supported_interface_orientations_ipad: &["UIInterfaceOrientationPortrait"],
    };
    plist::to_writer_xml(
        &mut File::create("target/ios/Sources/Info.plist").unwrap(),
        &infoplist,
    )
    .unwrap();

    let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
    let mut path = path.to_owned();
    path.pop();
    let path = path.to_str().unwrap().to_owned();

    let project = xcodegen::Project {
        name: spec.app.name.to_owned(),
        targets: vec![(
            spec.app.name.to_owned(),
            xcodegen::Target {
                product_type: xcodegen::ProductType::Application,
                platform: vec![xcodegen::Platform::IOS].into_iter().collect(),
                deployment_targets: vec![(xcodegen::Platform::IOS, "8.0".to_owned())]
                    .into_iter()
                    .collect(),
                sources: vec![xcodegen::TargetSource {
                    path: "Sources".to_owned(),
                }],
                settings: vec![
                    (
                        "PRODUCT_BUNDLE_IDENTIFIER".to_owned(),
                        spec.app.ios.bundle_identifier.clone(),
                    ),
                    ("LIBRARY_SEARCH_PATHS".to_owned(), path),
                    ("OTHER_LDFLAGS".to_owned(), "-ObjC -lc++".to_owned()),
                ]
                .into_iter()
                .collect(),
                dependencies: vec![xcodegen::Dependency::Framework {
                    framework: filename,
                    embed: false,
                }],
            },
        )]
        .into_iter()
        .collect(),
    };

    let file = File::create("target/ios/project.yml").unwrap();

    serde_yaml::to_writer(file, &project).unwrap();

    Command::new("xcodegen")
        .arg("generate")
        .arg("--spec")
        .arg("target/ios/project.yml")
        .arg("--project")
        .arg("target/ios")
        .stdout(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
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

pub fn run_ios_simulator(device: &simctl::Device, path: &Path) {
    let mut simctl = simctl::Simctl::new();

    let spec = Spec::open("Polyhorn.toml").unwrap();

    std::fs::create_dir_all("target/ios/").unwrap();

    let sim_id = device.identifier().to_owned();

    println!(
        "{} {}Generating Xcode project ...",
        style("[1/6]").bold().dim(),
        Emoji("⚡️  ", "")
    );

    if let Err(_) = generate_project_spec(&spec, path) {
        println!(
            "{} Couldn't find {}. Please run: {}.",
            style("[err]").bold().red(),
            style("`xcodegen`").bold().cyan(),
            style("`brew install xcodegen`").bold().cyan()
        );

        return;
    }

    println!(
        "{} {}Building Xcode project ...",
        style("[2/6]").bold().dim(),
        Emoji("🛠   ", "")
    );

    assert!(Command::new("xcrun")
        .arg("xcodebuild")
        .arg("-scheme")
        .arg(spec.app.name.clone() + "_iOS")
        .arg("-configuration")
        .arg("Debug")
        .arg("-destination")
        .arg("platform=iOS Simulator,name=".to_owned() + device.name())
        .arg("-derivedDataPath")
        .arg("derived-data")
        .stdout(Stdio::null())
        .current_dir("target/ios/")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());

    println!(
        "{} {}Booting iOS Simulator ...",
        style("[3/6]").bold().dim(),
        Emoji("📱  ", "")
    );

    if let Err(_) = simctl.boot(&sim_id) {
        println!("{} Couldn't boot simulator.", style("[err]").bold().red());

        return;
    }

    println!(
        "{} {}Opening iOS Simulator ...",
        style("[4/6]").bold().dim(),
        Emoji("📂  ", "")
    );

    Command::new("open")
        .arg("-a")
        .arg("Simulator.app")
        .stdout(Stdio::null())
        .current_dir("target/ios/")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!(
        "{} {}Installing on iOS Simulator ...",
        style("[5/6]").bold().dim(),
        Emoji("📲  ", "")
    );

    if let Err(_) = simctl.install(
        &sim_id,
        std::env::current_dir().unwrap().as_path().join(
            ("target/ios/derived-data/Build/Products/Debug-iphonesimulator/".to_owned()
                + &spec.app.name
                + ".app")
                .as_str(),
        ),
    ) {
        println!(
            "{} Couldn't install app on simulator.",
            style("[err]").bold().red()
        );

        return;
    }

    println!(
        "{} {}Launching on iOS Simulator ...",
        style("[6/6]").bold().dim(),
        Emoji("🚀️  ", "")
    );

    if let Err(_) = simctl.launch(&sim_id, &spec.app.ios.bundle_identifier) {
        println!(
            "{} Couldn't launch app on simulator.",
            style("[err]").bold().red()
        );

        return;
    }
}

pub fn install_ios_simulator_target() {
    println!(
        "{} {}Installing x86_64-apple-ios target ...",
        style("[1/8]").bold().dim(),
        Emoji("⚡️  ", "")
    );

    Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("x86_64-apple-ios")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn build_cargo() -> PathBuf {
    install_ios_simulator_target();

    println!(
        "{} {}Building runtime library with cargo ...",
        style("[2/8]").bold().dim(),
        Emoji("⚡️  ", "")
    );

    crate::core::cargo::build().unwrap()
}

pub fn build_assets() {
    println!(
        "{} {}Generating assets ...",
        style("[2/8]").bold().dim(),
        Emoji("⚡️  ", "")
    );

    let cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd.exec().unwrap();

    let mut result_path = std::env::current_dir().unwrap();
    result_path.push("target/ios/Sources/Assets.xcassets/");
    let _ = std::fs::create_dir_all(&result_path);

    {
        let mut result_path = result_path.clone();
        result_path.push("Contents.json");

        serde_json::to_writer_pretty(
            std::fs::File::create(result_path).unwrap(),
            &XcAssets {
                info: Info {
                    author: "polyhorn",
                    version: 1,
                },
            },
        )
        .unwrap();
    }

    {
        let mut result_path = result_path.clone();
        result_path.push("AppIcon.appiconset");
        let _ = std::fs::create_dir_all(&result_path);

        result_path.push("Contents.json");

        serde_json::to_writer_pretty(
            std::fs::File::create(result_path).unwrap(),
            &ImageSet {
                images: vec![
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "2x",
                        size: Some("20x20"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "3x",
                        size: Some("20x20"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "2x",
                        size: Some("29x29"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "3x",
                        size: Some("29x29"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "2x",
                        size: Some("40x40"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "3x",
                        size: Some("40x40"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "2x",
                        size: Some("60x60"),
                    },
                    Image {
                        filename: None,
                        idiom: "iphone",
                        scale: "3x",
                        size: Some("60x60"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "1x",
                        size: Some("20x20"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "2x",
                        size: Some("20x20"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "1x",
                        size: Some("29x29"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "2x",
                        size: Some("29x29"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "1x",
                        size: Some("40x40"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "2x",
                        size: Some("40x40"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "1x",
                        size: Some("76x76"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "2x",
                        size: Some("76x76"),
                    },
                    Image {
                        filename: None,
                        idiom: "ipad",
                        scale: "2x",
                        size: Some("83.5x83.5"),
                    },
                    Image {
                        filename: None,
                        idiom: "ios-marketing",
                        scale: "1x",
                        size: Some("1024x1024"),
                    },
                ],
                info: Info {
                    author: "polyhorn",
                    version: 1,
                },
            },
        )
        .unwrap();
    }

    for package in metadata.packages {
        let mut path = package.manifest_path.clone();
        path.pop();
        path.push("Polyhorn.toml");

        if !path.exists() {
            continue;
        }

        path.pop();
        path.push("assets");

        for entry in std::fs::read_dir(path).unwrap() {
            if let Ok(entry) = entry {
                if entry
                    .path()
                    .extension()
                    .map(|ext| ext == "svg")
                    .unwrap_or_default()
                {
                    let path = entry.path();
                    let name = path.file_stem().unwrap().to_str().unwrap();

                    let mut result_path = std::env::current_dir().unwrap();
                    result_path.push("target/ios/Sources/Assets.xcassets/".to_owned());
                    result_path.push(&package.name);
                    let _ = std::fs::create_dir_all(&result_path);

                    {
                        let mut result_path = result_path.clone();
                        result_path.push("Contents.json");

                        serde_json::to_writer_pretty(
                            std::fs::File::create(result_path).unwrap(),
                            &Folder {
                                info: Info {
                                    author: "polyhorn",
                                    version: 1,
                                },
                                properties: Properties {
                                    provides_namespace: true,
                                },
                            },
                        )
                        .unwrap();
                    }

                    result_path.push(name.to_owned() + ".imageset");
                    let _ = std::fs::create_dir_all(&result_path);

                    let zooms = [
                        ("1x", usvg::FitTo::Original),
                        ("2x", usvg::FitTo::Zoom(2.0)),
                        ("3x", usvg::FitTo::Zoom(3.0)),
                    ];

                    for &(suffix, zoom) in &zooms {
                        let mut result_path = result_path.clone();
                        result_path.push("image".to_owned() + "@" + suffix + ".png");

                        let tree =
                            usvg::Tree::from_file(entry.path(), &usvg::Options::default()).unwrap();
                        let image = resvg::render(&tree, zoom, None).unwrap();
                        image.save_png(result_path).unwrap();
                    }

                    result_path.push("Contents.json");

                    serde_json::to_writer_pretty(
                        std::fs::File::create(result_path).unwrap(),
                        &ImageSet {
                            images: vec![
                                Image {
                                    filename: Some("image@1x.png"),
                                    idiom: "universal",
                                    scale: "1x",
                                    size: None,
                                },
                                Image {
                                    filename: Some("image@2x.png"),
                                    idiom: "universal",
                                    scale: "2x",
                                    size: None,
                                },
                                Image {
                                    filename: Some("image@3x.png"),
                                    idiom: "universal",
                                    scale: "3x",
                                    size: None,
                                },
                            ],
                            info: Info {
                                author: "polyhorn",
                                version: 1,
                            },
                        },
                    )
                    .unwrap();
                }
            }
        }
    }
}
