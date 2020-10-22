use super::{IOSContext, IOSError};
use crate::core::{rasterize, Manager, Task};
use crate::ios::xcassets::{Folder, Image, ImageSet, Info, Properties, XcAssets};

/// This task generates an Xcode-compatible assets catalog for the assets of
/// the Polyhorn project that is being compiled and its dependencies.
pub struct GenerateXcassets;

impl Task for GenerateXcassets {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Generating"
    }

    fn message(&self) -> &str {
        "xcassets"
    }

    fn detail(&self) -> &str {
        "for iOS"
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let target_dir = context.config.target_dir.join("polyhorn-ios");

        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(context.config.manifest_dir.join("Cargo.toml"));
        let metadata = cmd.exec().unwrap();

        let mut result_path = std::env::current_dir().unwrap();
        result_path.push(target_dir.join("Sources/Assets.xcassets/"));
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
                        result_path.push(target_dir.join("Sources/Assets.xcassets/"));
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

                        let zooms = [("1x", 1.0), ("2x", 2.0), ("3x", 3.0)];

                        for &(suffix, zoom) in &zooms {
                            let mut result_path = result_path.clone();
                            result_path.push("image".to_owned() + "@" + suffix + ".png");

                            rasterize(&entry.path(), zoom, &result_path).unwrap();
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

        Ok(context)
    }
}
