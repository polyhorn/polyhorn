//! Types and functions that implement the `asset!(...)` macro.

use casco::stream::IntoTokenStream;
use quote::quote;

/// Implementation of the `asset!(...)` macro.
pub fn asset_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let stream = input.into_token_stream();
    let tokens = stream.into_iter().collect::<Vec<_>>();

    let name = match casco::domain::string(&tokens) {
        (Some(name), []) => name,
        _ => {
            return quote! {
                compile_error!("Invalid syntax for `asset!(...)` macro.")
            }
            .into()
        }
    };

    let package = std::env::var("CARGO_PKG_NAME").unwrap();

    let tree = usvg::Tree::from_file(
        format!(
            "{}/assets/{}.svg",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            name
        ),
        &usvg::Options::default(),
    )
    .expect(&format!(
        "Couldn't find asset: \"assets/{}.svg\" in package: {:?}",
        name,
        std::env::var("CARGO_PKG_NAME").unwrap(),
    ));

    let width = tree.svg_node().size.width() as f32;
    let height = tree.svg_node().size.height() as f32;

    (quote! {
        polyhorn::assets::ImageAsset::new(#package, #name, #width, #height)
    })
    .into()
}
