//! Bindings to Foundation.framework.
//!
//! ```objective-c
//! #import <Foundation/Foundation.h>
//! ```

mod array;
mod attributed_string;
mod number;
mod paragraph_style;
mod string;
mod value;

pub use array::NSMutableArray;
pub use attributed_string::{NSAttributedString, NSAttributes, NSMutableAttributedString};
pub use number::NSNumber;
pub use paragraph_style::{NSMutableParagraphStyle, NSParagraphStyle, NSTextAlignment};
pub use string::NSString;
pub use value::NSValue;
