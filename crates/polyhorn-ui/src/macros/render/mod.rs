//! Functions that implement the `render!(...)` macro.

use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

/// Implementation of the `render!(...)` macro for Android.
pub fn render_impl_android(input: TokenStream) -> TokenStream {
    quote! {
        #[allow(non_snake_case)]
        #[no_mangle]
        #[cfg(target_os = "android")]
        unsafe extern "C" fn Java_com_glacyr_polyhorn_Application_main(
            env: *mut std::ffi::c_void,
            _: *mut std::ffi::c_void,
            activity: *mut std::ffi::c_void,
        ) {
            let container = polyhorn::raw::OpaqueContainer::activity(env, activity);
            std::mem::forget(polyhorn::render(|| poly!(#input), container));
        }
    }
}

/// Implementation of the `render!(...)` macro for iOS.
pub fn render_impl_ios(input: TokenStream) -> TokenStream {
    quote! {
        #[no_mangle]
        #[cfg(target_os = "ios")]
        unsafe extern "C" fn __polyhorn_main() {
            std::mem::forget(polyhorn::render(
                || poly!(#input),
                polyhorn::raw::OpaqueContainer::root(),
            ));
        }
    }
}

/// Implementation of the `render!(...)` macro.
pub fn render_impl(input: TokenStream) -> TokenStream {
    // Since there's no good way to determine the target OS from within a proc
    // macro, we instead concatenate each output.
    TokenStream::from_iter(vec![
        render_impl_android(input.clone()),
        render_impl_ios(input.clone()),
    ])
}
