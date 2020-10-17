//! Types that represent the data structures of the `xcassets` format.

use serde::Serialize;

/// Represents the JSON-serialized data structure that is contained within a
/// `xcassets` asset catalog.
#[derive(Serialize)]
pub struct XcAssets<'a> {
    /// Contains metadata about this asset catalog.
    pub info: Info<'a>,
}

/// Represents the JSON-serialized data structure that is contained within an
/// image set in an asset catalog.
#[derive(Serialize)]
pub struct ImageSet<'a> {
    /// A set of alternative images for this image set. For example, multiple
    /// images can be provided to support different screen DPIs.
    pub images: Vec<Image<'a>>,

    /// Contains metadata about this image set.
    pub info: Info<'a>,
}

/// Metadata structure that is used in several places in an asset catalog.
#[derive(Serialize)]
pub struct Info<'a> {
    /// Contains the author of the corresponding data structure. This is usually
    /// Xcode.
    pub author: &'a str,

    /// Contains the version of the corresponding data structure. This is
    /// usually 1.
    pub version: usize,
}

/// Represents the JSON-serialized data structure that is contained within a
/// folder in an asset catalog.
#[derive(Serialize)]
pub struct Folder<'a> {
    /// Properties of this folder.
    pub properties: Properties,

    /// Contains metadata about this folder.
    pub info: Info<'a>,
}

/// Additional properties of a folder.
#[derive(Serialize)]
pub struct Properties {
    /// Boolean that indicates whether this folder provides a namespace. If it
    /// provides a namespace, images are accessed through
    /// `UIImage(named: "folder-name/image-name")`.
    #[serde(rename = "provides-namespace")]
    pub provides_namespace: bool,
}

/// A single image within an image set.
#[derive(Serialize)]
pub struct Image<'a> {
    /// Original file name of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,

    /// Contains the device idiom for which this image should be used. For
    /// example, this can be used to distinguish between iPhone and iPad.
    pub idiom: &'a str,

    /// Relative screen DPI for which this image should be used. For example,
    /// this can be used to provide both @1x and @2x (or even @3x)
    /// rasterizations of the same source image.
    pub scale: &'a str,

    /// Size of this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<&'a str>,
}
