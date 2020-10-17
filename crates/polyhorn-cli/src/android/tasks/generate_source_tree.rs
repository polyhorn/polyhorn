use std::fs::File;

use super::{AndroidContext, AndroidError};
use crate::core::{Manager, Task};
use crate::template::SourceTree;

/// This task generates a new source tree based on the template that ships with
/// `polyhorn-cli`.
pub struct GenerateSourceTree;

impl Task for GenerateSourceTree {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Generating"
    }

    fn message(&self) -> &str {
        "source tree"
    }

    fn detail(&self) -> &str {
        "for Android"
    }

    fn run(
        &self,
        context: AndroidContext,
        manager: &mut Manager,
    ) -> Result<AndroidContext, AndroidError> {
        let package_dir = context.config.spec.app.android.package.replace(".", "/");

        let mut tree = SourceTree::new();

        macro_rules! template {
            ($tree:ident, $path:literal) => {
                tree.template(
                    $path,
                    include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/android/template/",
                        $path,
                    )),
                );
            };
        }

        macro_rules! copy {
            ($tree:ident, $path:literal) => {
                tree.copy(
                    $path,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/android/template/",
                        $path,
                    )),
                );
            };
        }

        tree.template(
            &format!("app/src/main/java/{}/MainActivity.java", package_dir),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/android/template/",
                "app/src/main/java/com/example/myapplication/MainActivity.java"
            )),
        );

        copy!(tree, "build.gradle");
        copy!(tree, "gradle.properties");
        copy!(tree, "gradlew");
        copy!(tree, "gradlew.bat");
        template!(tree, "settings.gradle");
        template!(tree, "app/build.gradle");
        template!(tree, "app/src/main/AndroidManifest.xml");
        copy!(tree, "app/src/main/res/drawable/ic_launcher_background.xml");
        copy!(
            tree,
            "app/src/main/res/drawable-v24/ic_launcher_foreground.xml"
        );
        copy!(tree, "app/src/main/res/layout/activity_main.xml");
        copy!(tree, "app/src/main/res/layout/content_main.xml");
        template!(tree, "app/src/main/res/menu/menu_main.xml");
        copy!(
            tree,
            "app/src/main/res/mipmap-anydpi-v26/ic_launcher_round.xml"
        );
        copy!(tree, "app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml");
        copy!(tree, "app/src/main/res/mipmap-hdpi/ic_launcher_round.png");
        copy!(tree, "app/src/main/res/mipmap-hdpi/ic_launcher.png");
        copy!(tree, "app/src/main/res/mipmap-mdpi/ic_launcher_round.png");
        copy!(tree, "app/src/main/res/mipmap-mdpi/ic_launcher.png");
        copy!(tree, "app/src/main/res/mipmap-xhdpi/ic_launcher_round.png");
        copy!(tree, "app/src/main/res/mipmap-xhdpi/ic_launcher.png");
        copy!(tree, "app/src/main/res/mipmap-xxhdpi/ic_launcher_round.png");
        copy!(tree, "app/src/main/res/mipmap-xxhdpi/ic_launcher.png");
        copy!(
            tree,
            "app/src/main/res/mipmap-xxxhdpi/ic_launcher_round.png"
        );
        copy!(tree, "app/src/main/res/mipmap-xxxhdpi/ic_launcher.png");
        copy!(tree, "app/src/main/res/values/colors.xml");
        copy!(tree, "app/src/main/res/values/dimens.xml");
        copy!(tree, "app/src/main/res/values/strings.xml");
        copy!(tree, "app/src/main/res/values/themes.xml");
        copy!(tree, "app/src/main/res/values-night/themes.xml");
        copy!(tree, "app/proguard-rules.pro");
        copy!(tree, "gradle/wrapper/gradle-wrapper.jar");
        copy!(tree, "gradle/wrapper/gradle-wrapper.properties");

        let bar = manager.progress_bar(tree.len());

        let mut destination_path = context.config.target_dir.clone();
        destination_path.push("polyhorn-android");

        for result in tree.generate(&context.config.spec, &destination_path) {
            bar.inc(1);

            match result {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Encountered an error: {:#?}", error);
                    std::process::abort();
                }
            }
        }

        {
            use std::os::unix::fs::PermissionsExt;

            let mut gradlew_path = destination_path.clone();
            gradlew_path.push("gradlew");
            let gradlew = File::open(gradlew_path).unwrap();
            let mut permissions = gradlew.metadata().unwrap().permissions();
            permissions.set_mode(0o744);
            gradlew.set_permissions(permissions).unwrap();
        }

        bar.finish_and_clear();

        Ok(context)
    }
}
