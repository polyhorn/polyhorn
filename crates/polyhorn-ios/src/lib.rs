//! This crate implements Polyhorn for iOS.

#![warn(missing_docs)]

pub use polyhorn_core::{render, Context, ContextProvider, Key, Link, Reference, State};
pub use polyhorn_ui::{assets, color, font, geometry, layout, linalg, styles};
pub use polyhorn_ui_macros::render;

pub mod components;
pub mod handles;
pub mod prelude;
pub mod raw;

/// Re-exports of hooks provided by Polyhorn Core and Polyhorn UI.
pub mod hooks {
    pub use polyhorn_core::{
        use_async, use_context, use_effect, use_id, use_layout_effect, use_reference, use_state,
        UseAsync, UseContext, UseEffect, UseLayoutEffect, UseReference,
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

/// Polyhorn core weak type that is specialized for the iOS platform.
pub type Weak = polyhorn_core::Weak<Platform>;

/// Polyhorn core weak link type that is specialized for the iOS platform.
pub type WeakLink<'a> = polyhorn_core::WeakLink<'a, Platform>;

/// Polyhorn core weak reference type that is specialized for the iOS platform.
pub type WeakReference<T> = polyhorn_core::WeakReference<Platform, T>;

/// Polyhorn core weak state type that is specialized for the iOS platform.
pub type WeakState<T> = polyhorn_core::WeakState<Platform, T>;

pub use raw::Component;
