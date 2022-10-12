use proc_macro2::{TokenStream, token_stream, TokenTree, Punct, Ident};
use quote::{quote, TokenStreamExt};
use syn::{Token, token, parse::{Parse, ParseStream}, Expr, parse};
// use syn::{parse2,  DeriveInput};

pub struct KeyValue {
    pub key: Ident,
    pub colon: Token!(:),
    // pub string: Option<syn::LitStr>,
    // pub num: Option<syn::LitInt>,
    pub value: syn::ExprLit,
    pub comma: Option<Token!(,)>,
} impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            colon: input.parse()?,
            // string: input.parse()?,
            // num: input.parse()?,

            value: input.parse()?,
            comma: input.parse()?,
        })
    }
}

pub  struct KeyValues(Vec<KeyValue>);
impl Parse for KeyValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut key_values = KeyValues(vec![]);

        while !input.is_empty() {
            key_values.0.push(input.parse::<KeyValue>()?)    
        }

        Ok(key_values)
    }
}

pub fn atruct(stream: TokenStream) -> TokenStream {

    let kvs: KeyValues = parse(stream.into()).expect("line 42");

    let mut struct_def = TokenStream::new();
    let mut instance = TokenStream::new();

    struct_def.extend(quote!());

    for kv in kvs.0 {
        let key = kv.key;
        let value = kv.value;

        struct_def.extend(match &value.lit {
            syn::Lit::Str(str) => quote!(#key: &'static str, ),
            syn::Lit::Int(int) => quote!(#key: isize, ),
            syn::Lit::Float(float) => quote!(#key: f64, ),
            syn::Lit::Bool(bool) => quote!(#key: bool, ),
            _ => panic!("i don't support it")
        });
        instance.extend(
            quote!(#key: #value, )
        );
    }

    quote!({
        struct S1 {#struct_def}
        S1 {#instance}
    })
    
}
