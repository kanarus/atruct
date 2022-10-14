use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::parser::StructsData;


pub fn build_token_stream(structs_data: StructsData) -> TokenStream {
    let given_name = structs_data.name();

    let mut defs = TokenStream::new();
    for (id, s) in structs_data {
        let wrapping_name =
            if &id == "0" {quote!(#given_name)} else {wrapping_name(&id)};

        let mut field_defs = TokenStream::new();
        for (ident, value) in s.fields() {
            let type_name = value.get_type();
            field_defs.extend(quote!(
                #ident: #type_name,
            ))
        }
        defs.extend(quote!(
            struct #wrapping_name {#field_defs}
        ))
    }

    defs
}

pub fn wrapping_name(id: &String) -> TokenStream {
    let name = format_ident!("S_{}", id.clone());
    quote!(#name)
}