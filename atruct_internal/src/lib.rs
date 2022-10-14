use proc_macro2::TokenStream;
use quote::quote;
use syn::parse;


mod atruct_value;
pub fn atruct(stream: TokenStream) -> TokenStream {
    use atruct_value::{
        parser::{Atruct, StructMap},
        builder::build_token_stream,
    };

    let atruct: Atruct = parse(stream.into()).expect("failed to parse input to Atruct");
    let struct_map = StructMap::from_fields(atruct.into_fields());
    build_token_stream(struct_map)
}

mod atruct_type;
#[allow(non_snake_case)]
pub fn Atruct(stream: TokenStream) -> TokenStream {
    use atruct_type::{
        parser::{Atruct, StructsData},
        builder::build_token_stream,
    };

    let atruct: Atruct = parse(stream.into()).expect("failed to parse input to Atruct");
    let struct_map = StructsData::from_name_and_fields(atruct.name(), atruct.into_fields());
    build_token_stream(struct_map)
}

mod atruct_attribute;
#[allow(non_snake_case)]
pub fn Return(fields: TokenStream, function: TokenStream) -> TokenStream {
    use atruct_attribute::{
        parser::{Return, Function},
        builder::{build_return_struct, replace_marktoken_with_structname},
    };

    let ret: Return = parse(fields.into()).expect("failed to parse return fields");
    let func: Function = parse(function.into()).expect("failed to parse function");
    let struct_def = build_return_struct(&func.name, ret);
    let fn_def = replace_marktoken_with_structname(func);
    quote!(
        #struct_def
        #fn_def
    )
}
