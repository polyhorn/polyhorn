use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This task creates a universal binary from one or multiple
/// architecture-specific static libraries for iOS.
pub struct CreateUniversalBinary;

impl Task for CreateUniversalBinary {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Creating"
    }

    fn message(&self) -> &str {
        "Universal Binary"
    }

    fn detail(&self) -> &str {
        "for iOS"
    }

    fn run(
        &self,
        mut context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        if context.products.len() == 1 {
            let path = context.products.values().next().unwrap().to_owned();
            context.universal_binary_path = Some(path);
            Ok(context)
        } else {
            todo!("Creating universal binaries for iOS has not yet been implemented.")
        }
    }
}
