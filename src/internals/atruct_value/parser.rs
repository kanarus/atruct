use proc_macro2::{Ident, Span, Punct};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Colon, Brace, At, Paren, Bang, Sub},
    parse::{Parse, ParseStream},
    braced, Expr, Type, parenthesized, Error
};


pub struct Atruct {
    fields: Punctuated<Field, Comma>,
}
#[derive(Clone)]
pub struct Field {
    pub name:            Ident,
    pub type_annotation: Option<TypeAnnotation>,
    _colon:              Colon,
    pub content:         FieldContent,
}
#[derive(Clone)]
pub enum TypeAnnotation {
    AtAnnotation {
        _at:           At,
        type_of_value: Type,
    },
    ParenAnnotation {
        _paren:        Paren,
        type_of_valle: Type,
    },
}
#[derive(Clone)]
pub enum FieldContent {
    Value {
        prefix: Option<Prefix>,
        expr:   Expr,
    },
    Nest {
        _brace: Brace,
        fields: Punctuated<Field, Comma>,
    },
}
#[derive(Clone)]
pub enum Prefix {
    Bang,
    Minus,
}


impl Atruct {
    pub fn into_fields(self) -> Punctuated<Field, Comma> {
        self.fields
    }
} impl Parse for Atruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Atruct {
            fields: input.parse_terminated(Field::parse)?
        })
    }
} impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Field {
            name: input.parse().expect("expected identifier / not supporting block value like `field: {let v = Vec::new(); v}`"),
            type_annotation:
                if input.peek(At)
                || input.peek(Paren) {
                    Some(input.parse()?)
                } else {None},
            _colon:  input.parse()?,
            content: input.parse()?,
        })
    }
} impl Parse for TypeAnnotation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(At) {
            Ok(Self::AtAnnotation {
                _at:           input.parse()?,
                type_of_value: input.parse()?,
            })
        } else if input.peek(Paren) {
            let type_buf;
            Ok(Self::ParenAnnotation {
                _paren:        parenthesized!(type_buf in input),
                type_of_valle: type_buf.parse()?,
            })
        } else {
            Err(Error::new(Span::call_site(), "unexpected type annotation"))
        }
    }
} impl Parse for FieldContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields_buf;
        if input.peek(Brace) {
            Ok(Self::Nest {
                _brace: braced!(fields_buf in input),
                fields: fields_buf.parse_terminated(Field::parse)?,
            })
        } else {
            Ok(Self::Value {
                prefix:
                    if input.peek(Bang) {
                        input.parse::<Punct>().unwrap();
                        Some(Prefix::Bang)
                    } else if input.peek(Sub) {
                        input.parse::<Punct>().unwrap();
                        Some(Prefix::Minus)
                    } else {None},
                expr: input.parse()?
            })
        }
    }
}
