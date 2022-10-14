use proc_macro2::{Ident, TokenStream, TokenTree};
use syn::{punctuated::Punctuated, token::{Comma, Colon, Fn, Paren, RArrow, Brace, Rem}, Type, parse::Parse, parenthesized, braced};


pub struct Return {
    pub fields: Punctuated<Field, Comma>
}
pub struct Function {
    _fn:      Fn,
    pub name: Ident,
    _paren:   Paren,
    pub args: Punctuated<Field, Comma>,
    _arrow:   RArrow,
    _mark:    Rem,
    _brace:   Brace,
    pub body: TokenStream,
}
pub struct Field {
    pub name:   Ident,
    _colon: Colon,
    pub typexp: Type,
}
impl Parse for Return {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Field::parse)?
        })
    }
}
impl Parse for Function {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (args_buf, body_buf);
        Ok(Self {
            _fn:    input.parse()?,
            name:   input.parse()?,
            _paren: parenthesized!(args_buf in input),
            args:   args_buf.parse_terminated(Field::parse)?,
            _arrow: input.parse()?,
            _mark:   input.parse()?,
            _brace: braced!(body_buf in input),
            body:   TokenTree::parse(&body_buf)?.into(),
        })
    }
}
impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name:   input.parse()?,
            _colon: input.parse()?,
            typexp: input.parse()?,
        })
    }
}
