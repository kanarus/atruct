use proc_macro2::{TokenStream, Ident};
use quote::{quote, format_ident};
use crate::parser::StructMap;


pub fn build_token_stream(structs: StructMap) -> TokenStream {
    let mut defs = TokenStream::new();
    let mut instance = TokenStream::new();

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

    quote!({
        #defs
        #instance
    })
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
    use crate::parser::{StructMap, Struct, Value};
    use super::build_token_stream;

    #[test]
    fn build_a_1() {
        let case = StructMap::from_map(HashMap::from([(
            "0".to_owned(),
            Struct::from_map(
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