use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Colon, Brace, At, Paren},
    parse::{Parse, ParseStream},
    braced, Expr, Type, parenthesized, Error
};


pub struct Atruct {
    fields: Punctuated<Field, Comma>,
} pub struct Field {
    pub name:            Ident,
    pub type_annotation: Option<TypeAnnotation>,
    _colon:          Colon,
    pub content:         FieldContent,
} pub enum TypeAnnotation {
    AtAnnotation {
        _at:           At,
        type_of_value: Type,
    },
    ParenAnnotation {
        _paren:        Paren,
        type_of_valle: Type,
    },
} pub enum FieldContent {
    Value(Expr),
    Nest {
        _brace: Brace,
        fields: Punctuated<Field, Comma>,
    },
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
            name: input.parse()?,
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
            Ok(Self::Value(
                input.parse()?
            ))
        }
    }
}



/*
#[cfg(test)]
mod test {
    use proc_macro2::Span;
    use syn::{token::Colon, LitInt, LitStr};
    use super::*;

    #[test]
    fn parse_a_1() {
        let case = Punctuated::<Field, Comma>::from_iter([
            Field {
                name: Ident::new("a", Span::call_site()),
                _colon: Colon { spans: [Span::call_site()] },
                literal: Some(Lit::Int(LitInt::new("1", Span::call_site()))),
                nest: None,
            }
        ]);
        assert_eq!(
            StructMap::from_fields(case).0,
            StructMap::_from_map(HashMap::from([(
                "0".to_owned(),
                Struct(
                    HashMap::from([
                        (Ident::new("a", Span::call_site()),
                        Value::Int(1)),
                    ])
                )
            )])).0
        )
    }
    #[test]
    fn parse_a_1_b_string() {
        let case = Punctuated::<Field, Comma>::from_iter([
            Field {
                name: Ident::new("a", Span::call_site()),
                _colon: Colon { spans: [Span::call_site()] },
                literal: Some(Lit::Int(LitInt::new("1", Span::call_site()))),
                nest: None
            },
            Field {
                name: Ident::new("b", Span::call_site()),
                _colon: Colon { spans: [Span::call_site()] },
                literal: Some(Lit::Str(LitStr::new("string", Span::call_site()))),
                nest: None
            }
        ]);
        assert_eq!(
            StructMap::from_fields(case).0,
            StructMap::_from_map(HashMap::from([
                ("0".to_owned(),
                Struct(
                    HashMap::from([
                        (Ident::new("a", Span::call_site()),
                        Value::Int(1)),
                        (Ident::new("b", Span::call_site()),
                        Value::Str("string".to_owned())),
                    ])
                )),
            ])).0
        )
    }
}
*/