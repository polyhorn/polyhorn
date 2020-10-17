//! Types to work with info plists that are used on iOS to convey metadata of an
//! application. Note: the documentation of this module is taken directly from
//! Apple.

use serde::Serialize;
use smart_default::SmartDefault;

/// Data structure that contains the necessary metadata for apps running on iOS.
#[derive(Copy, Clone, Debug, SmartDefault, Eq, PartialEq, Serialize)]
pub struct InfoPlist<'a> {
    /// The default language and region for the bundle, as a language ID. Use
    /// the two-letter ISO 639-1 standard (preferred) or the three-letter ISO
    /// 639-2 standard. If an ISO 639-1 code isn't available for a particular
    /// language, use the ISO 639-2 code instead. There's no ISO 639-1 code for
    /// the Hawaiian language, so use the ISO 639-2 code. To distinguish between
    /// languages and regional dialects, use a language designator with a region
    /// designator and a script designator separated by hyphens. To specify the
    /// English language as it's used in the United Kingdom, use en-GB, where GB
    /// is the region designator. To represent Mandarin Chinese, spoken in
    /// Taiwan, and written in Traditional Chinese script, use zh-Hant-TW. To
    /// specify a script, combine a language designator with a script designator
    /// separated by a hyphen, as in az-Arab for Azerbaijani in the Arabic
    /// script.
    #[serde(rename = "CFBundleDevelopmentRegion")]
    pub bundle_development_region: &'a str,

    /// The name of the bundle's executable file.
    #[serde(rename = "CFBundleExecutable")]
    pub bundle_executable: &'a str,

    /// A unique identifier for a bundle. A bundle ID uniquely identifies a
    /// single app throughout the system. The bundle ID string must contain only
    /// alphanumeric characters (A-Z, a-z, and 0-9), hyphens (-), and periods
    /// (.).  The string should be in reverse-DNS format. Bundle IDs are case
    /// sensitive.
    #[serde(rename = "CFBundleIdentifier")]
    pub bundle_identifier: &'a str,

    /// The current version of the Information Property List structure. Xcode
    /// adds this key automatically. Don't change the value.
    #[serde(rename = "CFBundleInfoDictionaryVersion")]
    pub bundle_info_dictionary_version: &'a str,

    /// A user-visible short name for the bundle. This name can contain up to 15
    /// characters. The system may display it to users if `CFBundleDisplayName`
    /// isn't set.
    #[serde(rename = "CFBundleName")]
    pub bundle_name: &'a str,

    /// The type of bundle. This key consists of a four-letter code for the
    /// bundle type. For apps, the code is `APPL`, for framework, it's `FMWK`,
    /// and for bundles, it's `BNDL`. The default value is derived from the
    /// bundle extension or, if it can't be derived, the default value is
    /// `BNDL`.
    #[serde(rename = "CFBundlePackageType")]
    pub bundle_package_type: &'a str,

    /// The release or version number of the bundle. This key is a user-visible
    /// string for the version of the bundle. The required format is three
    /// period-separated integers, such as 10.14.1. The string can only contain
    /// numeric characters (0-9) and periods. Each integer provides information
    /// about the release in the format `[major].[minor].[patch]`. The key is
    /// used throughout the system to identify the version of the bundle.
    #[serde(rename = "CFBundleShortVersionString")]
    pub bundle_short_version_string: &'a str,

    /// The version of the build that identifies an iteration of the bundle.
    /// This key is a machine-readable string composed of one to three
    /// period-separated integers, such as 10.14.1. The string can only contain
    /// numeric characters (0-9) and periods. Each integer provides information
    /// about the build version in the format `[major].[minor].[patch]`. You can
    /// include more integers but the system ignores them. You can also
    /// abbreviate the build version by using only one or two integers, where
    /// missing integers in the format are interpreted as zeros. For example, 0
    /// specifies 0.0.0, 10 specifies 10.0.0, and 10.5 specifies 10.5.0. This
    /// key is required by the App Store and is used throughout the system to
    /// identify the version of the build. For macOS apps, increment the build
    /// version before you distribute a build.
    #[serde(rename = "CFBundleVersion")]
    pub bundle_version: &'a str,

    /// A Boolean value indicating whether the app must run in iOS.
    #[serde(rename = "LSRequiresIPhoneOS")]
    pub requires_iphone_os: bool,

    /// The filename of the storyboard from which to generate the app's launch
    /// image. Specify the name of the storyboard file without the filename
    /// extension. For example, if the filename of your storyboard is
    /// `LaunchScreen.storyboard`, specify "LaunchScreen" as the value for this
    /// key. If you prefer to configure your app's launch screen without
    /// storyboards, use `UILaunchScreen` instead.
    #[serde(rename = "UILaunchStoryboardName")]
    pub launch_storyboard_name: &'a str,

    /// The device-related features that your app requires to run.
    #[serde(rename = "UIRequiredDeviceCapabilities")]
    pub required_device_capabilities: &'a [DeviceCapability],

    /// The interface orientations the app supports.
    #[serde(rename = "UISupportedInterfaceOrientations")]
    pub supported_interface_orientations: &'a [InterfaceOrientation],

    /// The interface orientations the app supports on iPad.
    #[serde(rename = "UISupportedInterfaceOrientations~ipad")]
    pub supported_interface_orientations_ipad: &'a [InterfaceOrientation],
}

/// Device-related feature that an app may require to run.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum DeviceCapability {
    /// The presence of accelerometers. Use he Core Motion framework to receive
    /// accelerometer events. You don't need to include this value if your app
    /// detects only device orientation changes. Available in iOS 3.0 and later.
    Accelerometer,

    /// Support for ARKit. Available in iOS 11.0 and later.
    #[serde(rename = "arkit")]
    ARKit,

    /// Compilation for the armv7 instruction set, or as a 32/64-bit universal
    /// app. Available in iOS 3.1 and later.
    Armv7,

    /// Compilation for the arm64 instruction set. Include this key for all
    /// 64-bit apps and embedded bundles, like extensions and frameworks.
    /// Available in iOS 8.0 and later.
    Arm64,

    /// Autofocus capabilities in the device's still camera. You might need to
    /// include this value if your app supports macro photography or requires
    /// sharper images to perform certain image-processing tasks. Available in
    /// iOS 3.0 and later.
    #[serde(rename = "auto-focus-camera")]
    AutoFocusCamera,

    /// Bluetooth low-energy hardware. Available in iOS 5.0 and later.
    #[serde(rename = "bluetooth-le")]
    BluetoothLE,

    /// A camera flash. Use the `cameraFlashMode` property of a
    /// `UIImagePickerController` instance to control the camera's flash.
    /// Available in iOS 3.0 and later.
    #[serde(rename = "camera-flash")]
    CameraFlash,

    /// A forward-facing camera. Use the `cameraDevice` property of a
    /// `UIImagePickerController` instance to select the device's camera.
    /// Available in iOS 3.0 and later.
    #[serde(rename = "front-facing-camera")]
    FrontFacingCamera,

    /// Access to the Game Center service. Enable the Game Center capability in
    /// Xcode to add this value to your app. Available in iOS 4.1 and later.
    #[serde(rename = "gamekit")]
    GameKit,

    /// GPS (or AGPS) hardware for tracking locations. If you include this
    /// value, you should also include the location-services value. Require GPS
    /// only if your app needs location data more accurate than the cellular or
    /// Wi-Fi radios provide. Available in iOS 3.0 and later.
    GPS,

    /// A gyroscope. Use the Core Motion framework to retrieve information from
    /// gyroscope hardware. Available in iOS 3.0 and later.
    Gyroscope,

    /// Support for HealthKit. Available in iOS 8.0 and later.
    #[serde(rename = "healthkit")]
    HealthKit,

    /// Performance and capabilities of the A12 Bionic and later chips.
    /// Available in iOS 12.0 and later.
    #[serde(rename = "iphone-ipad-minimum-performance-a12")]
    IPhoneIPadMinimumPerformanceA12,

    /// Access to the device's current location using the Core Location
    /// framework. This value refers to the general location services feature.
    /// If you specifically need GPS-level accuracy, also include the gps
    /// feature. Available in iOS 3.0 and later.
    #[serde(rename = "location-services")]
    LocationServices,

    /// Magnetometer hardware. Apps use this hardware to receive heading-related
    /// events through the Core Location framework. Available in iOS 3.0 and
    /// later.
    Magnetometer,

    /// Support for graphics processing with Metal. Available in iOS 8.0 and
    /// later.
    Metal,

    /// The built-in microphone or accessories that provide a microphone.
    /// Available in iOS 3.0 and later.
    Microphone,

    /// Near Field Communication (NFC) tag detection and access to messages that
    /// contain NFC Data Exchange Format data. Use the Core NFC framework to
    /// detect and read NFC tags. Available in iOS 11.0 and later.
    NFC,

    /// The OpenGL ES 1.1 interface. Available in iOS 3.0 and later.
    #[serde(rename = "opengles-1")]
    OpenGLES1,

    /// The OpenGL ES 2.0 interface. Available in iOS 3.0 and later.
    #[serde(rename = "opengles-2")]
    OpenGLES2,

    /// The OpenGL ES 3.0 interface. Available in iOS 7.0 and later.
    #[serde(rename = "opengles-3")]
    OpenGLES3,

    /// Peer-to-peer connectivity over a Bluetooth network. Available in iOS 3.1
    /// and later.
    #[serde(rename = "peer-peer")]
    PeerPeer,

    /// The Messages app. You might require this feature if your app opens URLs
    /// with the sms scheme. Available in iOS 3.0 and later.
    SMS,

    /// A camera on the device. Use the `UIImagePickerController` interface to
    /// capture images from the device's still camera. Available in iOS 3.0 and
    /// later.
    #[serde(rename = "still-camera")]
    StillCamera,

    /// The Phone app. You might require this feature if your app opens URLs
    /// with the tel scheme. Available in iOS 3.0 and later.
    Telephony,

    /// A camera with video capabilities on the device. Use the
    /// `UIImagePickerController` interface to capture video from the device's
    /// camera. Available in iOS 3.0 and later.
    VideoCamera,

    /// Networking features related to Wi-Fi access. Available in iOS 3.0 and
    /// later.
    #[serde(rename = "wifi")]
    WiFi,
}

/// Represents a interface orientation that the app may support.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum InterfaceOrientation {
    /// The device is in portrait mode, with the device upright and the Home
    /// button on the bottom.
    #[serde(rename = "UIInterfaceOrientationPortrait")]
    Portrait,

    /// The device is in portrait mode but is upside down, with the device
    /// upright and the Home button at the top.
    #[serde(rename = "UIInterfaceOrientationPortraitUpsideDown")]
    PortraitUpsideDown,

    /// The device is in landscape mode, with the device upright and the Home
    /// button on the left.
    #[serde(rename = "UIInterfaceOrientationLandscapeLeft")]
    LandscapeLeft,

    /// The device is in landscape mode, with the device upright and the Home
    /// button on the right.
    #[serde(rename = "UIInterfaceOrientationLandscapeRight")]
    LandscapeRight,
}
