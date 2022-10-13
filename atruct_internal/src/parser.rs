use std::collections::{HashMap, hash_map::Iter};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::{Comma, Colon, self}, Lit, Type, Token};
use crate::{Field, builder::wrapping_name};


#[derive(Debug)]
pub struct StructMap(
    HashMap</*struct id*/String, Struct>
);
#[derive(Debug, PartialEq)]
pub struct Struct(
    HashMap<Ident/*field name*/, Value>
);
#[derive(Debug, PartialEq)]
pub enum Value {
    Int(isize),
    Bool(bool),
    Float(f64),
    Str(String),
    Struct(String/*id; pointer to another Struct in the StructList*/)
}

impl StructMap {
    pub fn from_fields(fields: Punctuated<Field, Comma>) -> Self {
        let mut map = HashMap::new();
        let init_id = "0".to_owned();
        
        parse_fields(init_id, fields, &mut map);

        // list.sort_unstable_by_key(|s| s.id);
        // list.reverse();
        StructMap(map)
    }
    pub fn from_map(map: HashMap<String, Struct>) -> Self {
        StructMap(map)
    }
}
impl Iterator for StructMap {
    type Item = (String, Struct);
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            let id = self.0.keys().last().unwrap().to_owned();
            let s = self.0.remove(&id).unwrap();
            Some((id, s))
        }
    }
}

impl Struct {
    pub fn fields(&self) -> Iter<Ident, Value> {
        self.0.iter()
    }
    pub fn from_map(field_map: HashMap<Ident, Value>) -> Self {
        Self(field_map)
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
    struct_map: &mut HashMap<String, Struct>,
) {
    let mut field_map = HashMap::new();
    let mut struct_fileds_count: usize = 0;

    for field in fields {
        let field_ident = field.name;

        if field.literal.is_some() {
            let literal = field.literal.unwrap();
            field_map.insert(field_ident, match &literal {
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
            field_map.insert(field_ident, Value::Struct(next_struct_id.clone()));
        
            parse_fields(next_struct_id, fields, struct_map);
        
        } else {
            panic!("no value is given")
        }
    }

    struct_map.insert(id, Struct(field_map));
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
            StructMap::from_fields(case).0,
            StructMap::from_map(HashMap::from([(
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
            StructMap::from_map(HashMap::from([
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