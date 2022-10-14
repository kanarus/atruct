use std::collections::{HashMap, hash_map::Iter};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::{Comma, Colon, Brace}, parse::{Parse, ParseStream}, braced, Type};

use super::builder::wrapping_name;


pub struct Atruct {
    name: Ident,
    _colon: Comma,
    fields: Punctuated<Field, Comma>,
} impl Atruct {
    pub fn name(&self) -> Ident {
        self.name.clone()
    }
    pub fn into_fields(self) -> Punctuated<Field, Comma> {
        self.fields
    }
}
impl Parse for Atruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Atruct {
            name: input.parse()?,
            _colon: input.parse()?,
            fields: { assert!(input.peek(Brace));
                      let _ = braced!(content in input);
                      content.parse_terminated(Field::parse)?
                    }
        })
    }
}
pub struct Field {
    name: Ident,
    _colon: Colon,
    typexp: Option<Type>,
    nest: Option<Punctuated<Field, Comma>>,
} impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Field {
            name: input.parse()?,
            _colon: input.parse()?,
            typexp: input.parse().ok(),
            nest: if input.peek(Brace) {
                      let _ = braced!(content in input);
                      content.parse_terminated(Field::parse).ok()
                  } else {
                      None
                  }
        })
    }
}


#[derive(Clone)]
pub struct StructsData {
    name: Ident,
    structs: HashMap</*struct id*/String, Struct>
}
#[derive(Clone)]
pub struct Struct(
    HashMap</*field name*/Ident, Value>
);
#[derive(Clone)]
pub enum Value {
    TypeExp(Type),
    Struct(String/*id of the nest struct*/),
}

impl StructsData {
    pub fn from_name_and_fields(name: Ident, structs: Punctuated<Field, Comma>) -> Self {
        let mut map = HashMap::new();
        let init_id = "0".to_owned();
        parse_fields(init_id, structs, &mut map);
        StructsData {name, structs: map}
    }
    pub fn _from_name_and_map(name: Ident, map: HashMap<String, Struct>) -> Self {
        StructsData {name, structs: map}
    }

    pub fn name(&self) -> Ident {
        self.name.clone()
    }
}
impl Iterator for StructsData {
    type Item = (String, Struct);
    fn next(&mut self) -> Option<Self::Item> {
        if self.structs.is_empty() {
            None
        } else {
            let id = self.structs.keys().last().unwrap().to_owned();
            let s = self.structs.remove(&id).unwrap();
            Some((id, s))
        }
    }
}

impl Struct {
    pub fn fields(&self) -> Iter<Ident, Value> {
        self.0.iter()
    }
    pub fn _from_map(field_map: HashMap<Ident, Value>) -> Self {
        Self(field_map)
    }
}

impl Value {
    pub fn get_type(&self) -> TokenStream {
        match self {
            Value::TypeExp(typexp) => quote!(#typexp),
            Value::Struct(id) => wrapping_name(id),
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

        if field.typexp.is_some() {
            let typexp = field.typexp.unwrap();
            field_map.insert(field_ident, Value::TypeExp(typexp));

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