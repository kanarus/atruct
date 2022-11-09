use proc_macro2::TokenStream;
use syn::{parse, parse2, Error};


trait Interpret<T> {
    fn interpret(self) -> T;
}
trait Build {
    fn build(self) -> TokenStream;
}


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
pub(super) fn Return(fields: TokenStream, function: TokenStream) -> Result<TokenStream, Error> {
    use atruct_return::{Return, ReturnFields, TargetFn};

    Ok(
        Return {
            fields: parse2::<ReturnFields>(fields)?,
            target: parse2::<TargetFn>(function)?,
        }.interpret().build()
    )
}
