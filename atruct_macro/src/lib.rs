use proc_macro::TokenStream;

#[proc_macro]
pub fn atruct(stream: TokenStream) -> TokenStream {
    atruct_internal::atruct(stream.into()).into()
}