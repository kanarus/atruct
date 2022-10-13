use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::{Comma, Colon, self}, Lit, Type, Token};
use crate::{Field, builder::wrapping_name};


#[derive(Debug, PartialEq)]
pub struct StructList(
    pub Vec<Struct>
);
#[derive(Debug, PartialEq)]
pub struct Struct{
    pub id: String,
    pub fields: HashMap<Ident/*field name*/, Value>
}
#[derive(Debug, PartialEq)]
pub enum Value {
    Int(isize),
    Bool(bool),
    Float(f64),
    Str(String),
    Struct(String/*id; pointer to another Struct in the StructList*/)
}

impl StructList {
    pub fn from_fields(fields: Punctuated<Field, Comma>) -> Self {
        let mut list = vec![];
        let init_id = "0".to_owned();
        
        parse_fields(init_id, fields, &mut list);

        // list.sort_unstable_by_key(|s| s.id);
        // list.reverse();
        StructList(list)
    }
}
impl Iterator for StructList {
    type Item = Struct;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Value {
    pub fn get_type(&self) -> TokenStream {
        use Value::*;
        match self {
            Int(_) => quote!(isize),
            Float(_) => quote!(f64),
            Bool(_) =>  quote!(bool),
            Str(_) =>  quote!(&'static str),
            Struct(id) => wrapping_name(id)
        }
    }
}


fn parse_fields(
    id: String,
    fields: Punctuated<Field, Comma>,
    struct_list: &mut Vec<Struct>,
) {
    let mut map = HashMap::new();
    let mut struct_fileds_count: usize = 0;

    for field in fields {
        let field_ident = field.name;

        if field.literal.is_some() {
            let literal = field.literal.unwrap();
            map.insert(field_ident, match &literal {
                Lit::Int(int) =>
                    Value::Int(int.base10_parse::<isize>().expect("not a integer")),
                Lit::Bool(boolean) =>
                    Value::Bool(boolean.value),
                Lit::Float(float) =>
                    Value::Float(float.base10_parse::<f64>().expect("not a float")),
                Lit::Str(string) =>
                    Value::Str(string.value()),
                _ => panic!("unsupported literal")
            });

        } else if field.nest.is_some() {
            struct_fileds_count += 1;
            let fields = field.nest.unwrap();
        
            let next_struct_id = format!("{id}_{struct_fileds_count}");
            map.insert(field_ident, Value::Struct(next_struct_id.clone()));
        
            parse_fields(next_struct_id, fields, struct_list);
        
        } else {
            panic!("no value is given")
        }
    }

    struct_list.push(
        Struct { id, fields: map }
    )
}


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
            StructList::from_fields(case),
            StructList(vec![
                Struct {
                    id: "0".to_owned(),
                    fields: HashMap::from([
                        (Ident::new("a", Span::call_site()),
                        Value::Int(1)),
                    ])
                }
            ])
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
            StructList::from_fields(case),
            StructList(vec![
                Struct {
                    id: "0".to_owned(),
                    fields: HashMap::from([
                        (Ident::new("a", Span::call_site()), Value::Int(1)),
                        (Ident::new("b", Span::call_site()), Value::Str("string".to_owned())),
                    ])
                }
            ])
        )
    }
}