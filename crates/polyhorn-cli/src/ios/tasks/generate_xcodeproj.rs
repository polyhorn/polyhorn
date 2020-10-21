use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::{Command, Stdio};

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};
use crate::ios::{infoplist, xcodegen};

/// This task generates an xcodeproj.
pub struct GenerateXcodeproj;

impl Task for GenerateXcodeproj {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Generating"
    }

    fn message(&self) -> &str {
        "Xcodeproj"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let target_dir = context.config.target_dir.join("polyhorn-ios");

        let _ = create_dir_all(&target_dir);
        let _ = create_dir_all(&target_dir.join("Sources"));

        let mut file = File::create(target_dir.join("Sources/main.m")).unwrap();
        file.write_all(&mut include_bytes!("../../../ios/template/main.m.tmpl").to_owned())
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
            required_device_capabilities: &[infoplist::DeviceCapability::Armv7],
            supported_interface_orientations: &[infoplist::InterfaceOrientation::Portrait],
            supported_interface_orientations_ipad: &[infoplist::InterfaceOrientation::Portrait],
        };
        plist::to_writer_xml(
            &mut File::create(target_dir.join("Sources/Info.plist")).unwrap(),
            &infoplist,
        )
        .unwrap();

        let path = context.universal_binary_path.as_ref().unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
        let mut path = path.to_owned();
        path.pop();
        let path = path.to_str().unwrap().to_owned();

        let project = xcodegen::Project {
            name: context.config.spec.app.name.to_owned(),
            targets: vec![(
                context.config.spec.app.name.to_owned(),
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
                            context.config.spec.app.ios.bundle_identifier.clone(),
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

        let file = File::create(target_dir.join("project.yml")).unwrap();

        serde_yaml::to_writer(file, &project).unwrap();

        Command::new("xcodegen")
            .arg("generate")
            .arg("--spec")
            .arg(target_dir.join("project.yml"))
            .arg("--project")
            .arg(&target_dir)
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(context)
    }
}
