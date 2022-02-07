use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn app(_: TokenStream, _: TokenStream) -> TokenStream {
    quote!(
        pub struct A;
    ).into()
}
