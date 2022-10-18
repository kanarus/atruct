use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::interpreter::{StructMap};


pub fn build_token_stream<T>(structs: StructMap<T>) -> TokenStream {
    let defs = build_struct_defs(structs.clone());
    let instance = build_struct_instance(structs);

    quote!({
        #defs
        #instance
    })
}

fn build_struct_defs<T>(structs: StructMap<T>) -> TokenStream {
    let mut defs = TokenStream::new();

    for (id, s) in structs {
        let wrapping_name = if &id == "0" {quote!(Atruct)} else {wrapping_name(&id)};

        let mut field_defs = TokenStream::new();
        for (ident, value) in s.fields() {
            let type_name = TokenStream::from_str(type_of(value).as_str()).expect("");
            field_defs.extend(quote!(#ident: #type_name, ));
        }
        defs.extend(quote!(
            struct #wrapping_name {#field_defs}
        ));
    }

    defs
}

fn build_struct_instance<T>(structs: StructMap<T>) -> TokenStream {
    build_struct_instance_inner(&structs, &"0".into(), &mut TokenStream::new())
}
fn build_struct_instance_inner<T>(
    structs: &StructMap<T>,
    id: &String,
    current_instance: &mut TokenStream
) -> TokenStream {
    let wrapping_name = if id == "0" {quote!(Atruct)} else {wrapping_name(&id)};

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
            Value::Value(value) => {
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

fn type_of<T>(_: &T) -> String {
    let mut raw_name = String::from(std::any::type_name::<T>());
    if raw_name.starts_with("alloc") {
        raw_name.replace_range(0..=4, "std")
    }
    raw_name
}

/*
#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use proc_macro2::{Ident, Span};
    use quote::quote;
    use crate::internals::atruct_value::parser::{StructMap, Struct, Value};
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
*/