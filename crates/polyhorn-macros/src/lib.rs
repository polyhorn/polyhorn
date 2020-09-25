mod poly_impl;

#[proc_macro]
pub fn poly(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    poly_impl::poly(input)
}
