use proc_macro2::TokenStream;
use quote::quote;
use syn::parse;


mod atruct_value;
pub(super) fn atruct(stream: TokenStream) -> TokenStream {
    use atruct_value::{
        parser::Atruct,
        interpreter::StructMap,
        builder::build_token_stream,
    };

    let atruct: Atruct = parse(stream.into()).expect("failed to parse input to Atruct");
    let struct_map = StructMap::from_fields(atruct.into_fields());
    build_token_stream(struct_map)
}


mod atruct_return;
#[allow(non_snake_case)]
pub(super) fn Return(fields: TokenStream, function: TokenStream) -> TokenStream {
    use atruct_return::{
        parser::{Return, Function},
        builder::{build_return_struct, replace_marktoken_with_structname},
    };

    let ret: Return = parse(fields.into()).expect("failed to parse return fields");
    let func: Function = parse(function.clone().into()).expect("failed to parse function");
    let struct_def = build_return_struct(&func.name, ret);
    let fn_def = replace_marktoken_with_structname(func);
    quote!(
        #struct_def
        #fn_def
    )
}
