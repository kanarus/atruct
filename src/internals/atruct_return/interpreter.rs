use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident, ToTokens};
use crate::internals::Interpret;
use super::Return;


pub(crate) struct ReturnStreams {
    pub struct_stream:   TokenStream,
    pub function_stream: TokenStream,
}

impl Interpret<ReturnStreams> for Return {
    fn interpret(self) -> ReturnStreams {
        let struct_name = camel_cased(&self.target.name);

        let struct_stream = {
            let mut return_fields = TokenStream::new();
            for field in self.fields {
                let (name, typexp) = (field.name, field.typexp);
                return_fields.extend(quote!(
                    #name: #typexp,
                ))
            }

            quote!(
                struct #struct_name {
                    #return_fields
                }
            )
        };

        let function_stream = {
            let taget = self.target;

            let args = taget.args.to_token_stream();
            let body = taget.body;

            let mut signature = TokenStream::new();
            for attr in taget.attrs {
                attr.to_tokens(&mut signature)
            }
            if let Some(vis) = taget.vis {
                vis.to_tokens(&mut signature)
            }
            if let Some(_async) = taget._async {
                _async.to_tokens(&mut signature)
            }
            if let Some(_unsafe) = taget._unsafe {
                _unsafe.to_tokens(&mut signature)
            }
            taget._fn.to_tokens(&mut signature);
            taget.name.to_tokens(&mut signature);
            if let Some(generics) = taget.generics {
                generics.to_tokens(&mut signature)
            }

            quote!(
                #signature(#args) -> #struct_name {
                    type Return = #struct_name;
                    #body
                }
            )
        };

        ReturnStreams { struct_stream, function_stream }
    }
}

fn camel_cased(function_name: &Ident) -> Ident {
    let mut struct_name = String::new();
    let mut is_head_of_word = true;
    for ch in function_name.to_string().chars() {
        if ch == '_' { is_head_of_word = true } else {
            struct_name.push(
                if is_head_of_word {is_head_of_word = false; ch.to_ascii_uppercase()} else {ch}
            )
        }
    }
    format_ident!("{struct_name}")
}