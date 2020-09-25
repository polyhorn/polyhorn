use serde::Serialize;

#[derive(Serialize)]
pub struct XcAssets<'a> {
    pub info: Info<'a>,
}

#[derive(Serialize)]
pub struct ImageSet<'a> {
    pub images: Vec<Image<'a>>,
    pub info: Info<'a>,
}

#[derive(Serialize)]
pub struct Info<'a> {
    pub author: &'a str,
    pub version: usize,
}

#[derive(Serialize)]
pub struct Folder<'a> {
    pub info: Info<'a>,
    pub properties: Properties,
}

#[derive(Serialize)]
pub struct Properties {
    #[serde(rename = "provides-namespace")]
    pub provides_namespace: bool,
}

#[derive(Serialize)]
pub struct Image<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,

    pub idiom: &'a str,
    pub scale: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<&'a str>,
}
