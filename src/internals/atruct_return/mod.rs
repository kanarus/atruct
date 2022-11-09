use proc_macro2::{TokenStream, Ident};
use syn::{punctuated::Punctuated, token, Type, Attribute, Visibility, Generics, FnArg, ItemImpl};

mod parser;
mod interpreter;
mod builder;


pub(super) struct WithReturn(
    ItemImpl
);


pub(super) struct Return {
    pub fields: ReturnFields,
    pub target: TargetFn,
}

pub(super) struct ReturnFields(
    Punctuated<ReturnField, token::Comma>
);
pub(super) struct ReturnField { // field def with NO visivility or else
    pub name:   Ident,
    _colon: token::Colon,
    pub typexp: Type,
}
pub(super) struct TargetFn { // function def with NO return type
    attrs:    Vec<Attribute>,
    vis:      Option<Visibility>,
    _async:   Option<token::Async>,
    _unsafe:  Option<token::Unsafe>,
    _fn:      token::Fn,
    name:     Ident,
    generics: Option<Generics>,
    _paren:   token::Paren,
    args:     Punctuated<FnArg, token::Comma>,
    _brace:   token::Brace,
    body:     TokenStream,
}


impl Iterator for ReturnFields {
    type Item = ReturnField;
    fn next(&mut self) -> Option<Self::Item> {
        let last = self.0.pop();
        let Some(last_pair) = last else { return None };
        Some(last_pair.into_value())
    }
}