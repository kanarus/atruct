use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use crate::atruct_value::parser::{StructMap, Value};


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
        let wrapping_name = wrapping_name(&id);

        let mut field_defs = TokenStream::new();
        for (ident, value) in s.fields() {
            let type_name = value.get_type();
            field_defs.extend(quote!(#ident: #type_name, ));
        }
        defs.extend(quote!(
            struct #wrapping_name {#field_defs}
        ));
    }

    defs
}

fn build_struct_instance(structs: StructMap) -> TokenStream {
    build_struct_instance_inner(&structs, &"0".into(), &mut TokenStream::new())
}
fn build_struct_instance_inner(
    structs: &StructMap,
    id: &String,
    current_instance: &mut TokenStream
) -> TokenStream {
    let wrapping_name = wrapping_name(id);

    let mut fields = TokenStream::new();
    let target = structs.get(id);
    for (name, value) in target.fields() {
        fields.extend(quote!(#name: ));
        match value {
            Value::Struct(next_id) => {
                let next_struct = build_struct_instance_inner(
                    structs, next_id, current_instance
                );
                fields.extend(quote!(#next_struct,))
            }
            other => {
                let literal_token = other.unwrap_literal_as_token();
                fields.extend(quote!(#literal_token,))
            }
        }
    }

    quote!(#wrapping_name {#fields})
}

pub fn wrapping_name(id: &String) -> TokenStream {
    let name = format_ident!("S_{}", id.clone());
    quote!(#name)
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use proc_macro2::{Ident, Span};
    use quote::quote;
    use crate::atruct_value::parser::{StructMap, Struct, Value};
    use super::build_token_stream;

    #[test]
    fn build_a_1() {
        let case = StructMap::_from_map(HashMap::from([(
            "0".to_owned(),
            Struct::_from_map(
                HashMap::from([
                    (Ident::new("a", Span::call_site()),
                    Value::Int(1)),
                ])
            )
        )]));

        assert_eq!(
            build_token_stream(case).to_string(),
            quote!(
                {struct S_0 {a: isize, }}
            ).to_string()
        )
    }
}