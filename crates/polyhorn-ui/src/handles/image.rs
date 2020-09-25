use crate::queries::ImageQuery;

/// This trait is implemented by platform-specific Image handles.
pub trait ImageHandle: ImageQuery {}
