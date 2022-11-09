use syn::{parse::Parse, Attribute, parenthesized, FnArg, braced};
use super::{ReturnFields, ReturnField, TargetFn, WithReturn};


impl Parse for WithReturn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}


impl Parse for ReturnFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse_terminated(ReturnField::parse)?))
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
            vis:      input.parse()?,
            _async:   input.parse()?,
            _unsafe:  input.parse()?,
            _fn:      input.parse()?,
            name:     input.parse()?,
            generics: input.parse()?,
            _paren:   parenthesized!(args_buf in input),
            args:     args_buf.parse_terminated(FnArg::parse)?,
            _brace:   braced!(body_buf in input),
            body:     body_buf.parse()?,
        })
    }
}
