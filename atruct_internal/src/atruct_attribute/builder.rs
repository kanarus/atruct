use proc_macro2::{TokenStream, Ident, TokenTree};
use quote::{format_ident, quote};

use super::parser::{Function, Return};

pub fn build_return_struct(func_name: &Ident, ret: Return) -> TokenStream {
    let name = get_struct_name(func_name);

    let mut fields = TokenStream::new();
    for field in ret.fields {
        let (field_name, field_type) = (field.name, field.typexp);
        fields.extend(quote!(
            #field_name: #field_type
        ))
    }

    quote!(
        struct #name {#fields}
    )
}

pub fn replace_marktoken_with_structname(func: Function) -> TokenStream {
    let fn_name = func.name;
    let struct_name= get_struct_name(&fn_name);

    let mut replcaed_body = TokenStream::new();
    let mut body = func.body.into_iter();
    while let Some(tt) = body.next() {
        replcaed_body.extend(match &tt {
            TokenTree::Punct(p) if p.as_char()=='%' => quote!(#struct_name),
            _ => quote!(#tt)
        })
    }

    let mut args_stream = TokenStream::new();
    for field in func.args {
        let (field_name, field_type) = (field.name, field.typexp);
        args_stream.extend(quote!(
            #field_name: #field_type, 
        ))
    }

    quote!(
        fn #fn_name(#args_stream) -> #struct_name {
            #replcaed_body
        }
    )
}
/*
fn skip_past_next_at(input: ParseStream) -> Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '@' => {
                    return Ok(((), next));
                }
                _ => rest = next,
            }
        }
        Err(cursor.error("no `@` was found after this point"))
    })
}
*/


fn get_struct_name(func_name: &Ident) -> Ident {
    let mut struct_name = String::new();

    let func_name = func_name.to_string();
    let mut func_name = func_name.chars();
    let mut flag = true;
    while let Some(c) = func_name.next() {
        if c == '_' {flag = true}
        else {struct_name.push(
            if flag {c.to_ascii_uppercase()} else {c}
        )}
    }

    format_ident!("{}", struct_name)
}