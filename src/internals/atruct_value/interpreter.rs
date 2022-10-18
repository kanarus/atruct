use std::collections::{HashMap, hash_map::Iter};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};
use syn::{punctuated::Punctuated, token::Comma, Expr, Lit, Type, ExprLit};
use super::{
    parser::*, utils::type_of_num,
};


pub struct StructMap(
    HashMap</*struct id*/Ident, Struct>
); pub struct Struct(
    HashMap</*field name*/Ident, Item>,
); pub enum Item {
    Value {
        value:         TokenStream,
        type_of_value: TokenStream,
    },
    Struct {
        id:     Ident,
        fields: Punctuated<Field, Comma>,
    }
}


impl StructMap {
    pub fn from_fields(fields: Punctuated<Field, Comma>) -> Self {
        let mut map = HashMap::new();
        let init_id = format_ident!("0");
        parse_fields(init_id, fields, &mut map);
        StructMap(map)
    }
    pub fn _from_map(map: HashMap<Ident, Struct>) -> Self {
        StructMap(map)
    }

    pub fn get(&self, id: &Ident) -> &Struct {
        self.0.get(id).expect("not found Struct")
    }
} impl Iterator for StructMap {
    type Item = (Ident, Struct);
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
    pub fn fields(&self) -> Iter<Ident, Item> {
        self.0.iter()
    }
    pub fn _from_map(field_map: HashMap<Ident, Item>) -> Self {
        Self(field_map)
    }
}

impl FieldContent {
    pub fn get_literal_type(literal: ExprLit) -> TokenStream {
        match literal.lit {
            Lit::Bool(boolean) => quote!(bool),
            Lit::Char(c) => quote!(char),
            Lit::Str(str) => quote!(&'static str),
            Lit::Float(float) => {
                let float_string = float.to_string();
                if float_string.contains("f32") {
                    quote!(f32)
                } else {
                    quote!(f64)
                }
            },
            Lit::Int(int) => {
                let int_string = int.to_string();
                if int_string.contains("usize") {
                    quote!(usize)
                } else if int_string.contains("isize") {
                    quote!(isize)
                } else if int_string.contains("u8") {
                    quote!(u8)
                } else if int_string.contains("u16") {
                    quote!(u16)
                } else if int_string.contains("u32") {
                    quote!(u32)
                } else if int_string.contains("u64") {
                    quote!(u64)
                } else if int_string.contains("u128") {
                    quote!(u128)
                } else if int_string.contains("i8") {
                    quote!(i8)
                } else if int_string.contains("i16") {
                    quote!(i16)
                } else if int_string.contains("i64") {
                    quote!(i64)
                } else if int_string.contains("i128") {
                    quote!(i128)
                } else {
                    quote!(i32)
                }
            },
            _ => panic!("supporing literal types: bool, char, &str, float, integer")
        }
    }
}
impl TypeAnnotation {
    fn get_type(&self) -> TokenStream {
        match self {
            TypeAnnotation::AtAnnotation {
                _at,
                type_of_value
            } => quote!(#type_of_value),
            TypeAnnotation::ParenAnnotation {
                _paren,
                type_of_valle
            } => quote!(#type_of_valle),
        }
    }
}


fn parse_fields(
    id: Ident,
    fields: Punctuated<Field, Comma>,
    struct_map: &mut HashMap<Ident, Struct>,
) {
    let mut field_map = HashMap::new();
    let mut struct_fileds_count: usize = 0;

    for field in fields {
        let field_name = field.name;
        match field.content {
            FieldContent::Value(value) => {
                let type_of_value = match value {
                    Expr::Lit(lit) => FieldContent::get_literal_type(lit),
                    _ => field.type_annotation.expect("type annotation is needed for non literal values").get_type(),
                };

                let err = field_map.insert(
                    field_name, Item::Value {
                        value: quote!(#value),
                        type_of_value
                    }
                ); assert!(err.is_none(),
                    "field {} already exists in the same class", field_name
                )
            },
            FieldContent::Nest {
                _brace,
                fields
            } => {
                struct_fileds_count += 1;
                let next_struct_id = format_ident!("{id}_{struct_fileds_count}");

                let err = field_map.insert(
                    field_name, Item::Struct {
                        id: next_struct_id,
                        fields
                    }
                ); assert!(err.is_none(),
                    "field {} already exists in the same class", field_name
                );

                parse_fields(next_struct_id, fields, struct_map)
            },
        }
    }

    struct_map.insert(id, Struct(field_map));
}
