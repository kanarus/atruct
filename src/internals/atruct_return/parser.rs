use syn::{parse::Parse, token, Attribute, parenthesized, FnArg, braced};
use super::{ReturnFields, ReturnField, TargetFn};


impl Parse for ReturnFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(
            input.parse_terminated(ReturnField::parse)?
        ))
    }
}
impl Parse for ReturnField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name:   input.parse()?,
            _colon: input.parse()?,
            typexp: input.parse()?
        })
    }
}

impl Parse for TargetFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (args_buf, body_buf);
        Ok(Self {
            attrs:    input.call(Attribute::parse_inner)?,
            vis:      if input.peek(token::Pub) {Some(input.parse()?)} else {None},
            _async:   input.parse()?,
            _unsafe:  input.parse()?,
            _fn:      input.parse()?,
            name:     input.parse()?,
            generics: if input.peek(token::Lt) {Some(input.parse()?)} else {None},
            _paren:   parenthesized!(args_buf in input),
            args:     args_buf.parse_terminated(FnArg::parse)?,
            _brace:   braced!(body_buf in input),
            body:     body_buf.parse()?,
        })
    }
}
