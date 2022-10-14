use proc_macro2::TokenStream;
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