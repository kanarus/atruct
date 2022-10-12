use proc_macro2::TokenStream;
use quote::quote;
use crate::parser::StructList;


pub fn build_token_stream(structs: StructList) -> TokenStream {
    let mut defs = TokenStream::new();
    let mut instance = TokenStream::new();

    for s in structs {
        let wrapping_name = wrapping_name(s.id);

        let mut field_defs = TokenStream::new();
        for (ident, value) in s.fields.iter() {
            let type_name = value.get_type();
            field_defs.extend(quote!(#ident: #type_name, ));
        }
        defs.extend(quote!(
            #wrapping_name {#field_defs}
        ));
    }

    quote!({
        #defs
        #instance
    })
}

pub fn wrapping_name(id: usize) -> String {
    format!("S{id}")
}