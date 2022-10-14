use proc_macro::TokenStream;


#[proc_macro]
pub fn atruct(stream: TokenStream) -> TokenStream {
    atruct_internal::atruct(stream.into()).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Atruct(stream: TokenStream) -> TokenStream {
    atruct_internal::Atruct(stream.into()).into()
}