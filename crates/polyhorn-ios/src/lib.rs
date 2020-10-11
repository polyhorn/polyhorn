//! This crate implements Polyhorn for iOS.

#![warn(missing_docs)]

pub use polyhorn_core::{render, Context, ContextProvider, Key, Reference, State};
pub use polyhorn_ui::{assets, color, font, geometry, layout, linalg, styles};

pub mod components;
pub mod handles;
pub mod prelude;
pub mod raw;

/// Re-exports of hooks provided by Polyhorn Core and Polyhorn UI.
pub mod hooks {
    pub use polyhorn_core::{
        use_async, use_context, use_effect, use_id, use_reference, use_state, UseAsync, UseContext,
        UseEffect, UseReference,
    };
    pub use polyhorn_ui::hooks::*;
}

use raw::Platform;

/// Polyhorn core element type that is specialized for the iOS platform.
pub type Element = polyhorn_core::Element<Platform>;

/// Polyhorn core instance type that is specialized for the iOS platform.
pub type Instance = polyhorn_core::Instance<Platform>;

/// Polyhorn core manager type that is specialized for the iOS platform.
pub type Manager<'a> = polyhorn_core::Manager<'a, Platform>;

pub use raw::Component;
