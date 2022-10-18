use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::interpreter::{StructMap, Item};


pub fn build_token_stream(structs: StructMap) -> TokenStream {
    let defs = build_struct_defs(structs.clone());
    let instance = build_struct_instance(structs);

    quote!({
        #defs
        #instance
    })
}

fn build_struct_defs(structs: StructMap) -> TokenStream {
    let mut defs = TokenStream::new();

    for (id, s) in structs {
        let wrapping_name = if &id == "0" {quote!(Atruct)} else {wrapping_name(&id)};

        let mut field_defs = TokenStream::new();
        for (ident, item) in s.fields() {
            let type_name = item.type_name();
            field_defs.extend(quote!(#ident: #type_name, ));
        }
        defs.extend(quote!(
            struct #wrapping_name {#field_defs}
        ));
    }

    defs
}

fn build_struct_instance(structs: StructMap) -> TokenStream {
    build_struct_instance_inner(&structs, &"0".to_owned(), &mut TokenStream::new())
}
fn build_struct_instance_inner(
    structs: &StructMap,
    id: &String,
    current_instance: &mut TokenStream
) -> TokenStream {
    let wrapping_name = if id == "0" {quote!(Atruct)} else {wrapping_name(id)};

    let mut fields = TokenStream::new();
    let target = structs.get(id);
    for (name, item) in target.fields() {
        fields.extend(quote!(#name: ));
        match item {
            Item::Struct(next_id) => {
                let next_struct = build_struct_instance_inner(
                    structs, next_id, current_instance
                );
                fields.extend(quote!(#next_struct, ))
            }
            Item::Value {
                #[allow(unused)]
                type_of_value,
                value,
            } => {
                fields.extend(quote!(#value, ))
            }
        }
    }

    quote!(#wrapping_name {#fields})
}

pub fn wrapping_name(id: &String) -> TokenStream {
    let name = format_ident!("S_{}", id.clone());
    quote!(#name)
}
