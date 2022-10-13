use proc_macro2::{TokenStream, Ident};
use syn::{Token, token::{Comma, Colon, Brace}, parse::{Parse, ParseStream}, parse, braced, punctuated::Punctuated, ExprLit, Lit};

mod parser; use parser::StructList;
mod builder; use builder::build_token_stream;


struct Atruct {
    fields: Punctuated<Field, Comma>,
} impl Parse for Atruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Atruct {
            fields: input.parse_terminated(Field::parse)?
        })
    }
}
pub struct Field {
    name: Ident,
    _colon: Colon,
    literal: Option<Lit>,
    // _brace: Option<Brace>,// Option<token::Brace>,
    nest: Option<Punctuated<Field, Comma>>,
} impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Field {
            name: input.parse()?,
            _colon: input.parse()?,
            literal: input.parse().ok(),
            // _brace: input., // braced!(content in input),
            nest: if input.peek(Brace) {
                    let _ = braced!(content in input);
                    content.parse_terminated(Field::parse).ok()
                } else {
                    None
                }
        })
    }
}

pub fn atruct(stream: TokenStream) -> TokenStream {
    let atruct: Atruct = parse(stream.into()).expect("failed to parse input to Atruct");
    let struct_list = StructList::from_fields(atruct.fields);
    build_token_stream(struct_list)
}
