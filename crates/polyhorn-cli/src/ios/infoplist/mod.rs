use serde::Serialize;
use smart_default::SmartDefault;

#[derive(Copy, Clone, Debug, SmartDefault, Eq, PartialEq, Serialize)]
pub struct InfoPlist<'a> {
    #[serde(rename = "CFBundleDevelopmentRegion")]
    pub bundle_development_region: &'a str,

    #[serde(rename = "CFBundleExecutable")]
    pub bundle_executable: &'a str,

    #[serde(rename = "CFBundleIdentifier")]
    pub bundle_identifier: &'a str,

    #[serde(rename = "CFBundleInfoDictionaryVersion")]
    pub bundle_info_dictionary_version: &'a str,

    #[serde(rename = "CFBundleName")]
    pub bundle_name: &'a str,

    #[serde(rename = "CFBundlePackageType")]
    pub bundle_package_type: &'a str,

    #[serde(rename = "CFBundleShortVersionString")]
    pub bundle_short_version_string: &'a str,

    #[serde(rename = "CFBundleVersion")]
    pub bundle_version: &'a str,

    #[serde(rename = "LSRequiresIPhoneOS")]
    pub requires_iphone_os: bool,

    #[serde(rename = "UILaunchStoryboardName")]
    pub launch_storyboard_name: &'a str,

    #[serde(rename = "UIRequiredDeviceCapabilities")]
    pub required_device_capabilities: &'a [&'a str],

    #[serde(rename = "UISupportedInterfaceOrientations")]
    pub supported_interface_orientations: &'a [&'a str],

    #[serde(rename = "UISupportedInterfaceOrientations~ipad")]
    pub supported_interface_orientations_ipad: &'a [&'a str],
}
