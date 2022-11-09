use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident, ToTokens};
use syn::{ImplItem, parse2, Error, ItemFn, Attribute};
use crate::internals::Interpret;
use super::{Return, WithReturn};


pub(crate) struct WithReturnStreams {
    pub impl_block_stream: TokenStream,
    pub structs_stream:    TokenStream,
}

impl Interpret<Result<WithReturnStreams, Error>> for WithReturn {
    fn interpret(self) -> Result<WithReturnStreams, Error> {
        let impl_block = self.0;
        let mut impl_block_stream = TokenStream::new();
        let mut structs_stream = TokenStream::new();
        
        for attr in impl_block.attrs {
            attr.to_tokens(&mut impl_block_stream)
        }
        if let Some(_unsafe) = impl_block.unsafety {
            _unsafe.to_tokens(&mut impl_block_stream)
        }
        impl_block.impl_token.to_tokens(&mut impl_block_stream);
        impl_block.generics.to_tokens(&mut impl_block_stream);
        if let Some((bang, path, _for)) = impl_block.trait_ {
            if let Some(_bang) = bang {
                _bang.to_tokens(&mut impl_block_stream)
            }
            path.to_tokens(&mut impl_block_stream);
            _for.to_tokens(&mut impl_block_stream);
        }
        impl_block.self_ty.to_tokens(&mut impl_block_stream);
        

        let mut impl_items = TokenStream::new();
        let return_attr_ident = format_ident!("Return");

        for item in impl_block.items {
            match item {
                ImplItem::Method(mut function) => {
                    if let Some(return_attr) = extract_attr_of_ident(
                        &return_attr_ident, &mut function.attrs
                    ) {
                        let ReturnStreams {
                            struct_stream,
                            function_stream
                        } = Return {
                            fields: return_attr.parse_args()?,
                            target: parse2(ItemFn {
                                attrs: function.attrs,
                                vis:   function.vis,
                                sig:   function.sig,
                                block: Box::new(function.block),
                            }.into_token_stream())?,
                        }.interpret();
                        
                        struct_stream.to_tokens(&mut structs_stream);
                        function_stream.to_tokens(&mut impl_items);
                    } else {
                        function.to_tokens(&mut impl_items)
                    }
                }
                other => other.to_tokens(&mut impl_items)
            }
        }

        impl_block_stream.extend(quote!(
            { #impl_items }
        ));

        Ok(WithReturnStreams { impl_block_stream, structs_stream })
    }
}


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
            taget.vis.to_tokens(&mut signature);
            if let Some(_async) = taget._async {
                _async.to_tokens(&mut signature)
            }
            if let Some(_unsafe) = taget._unsafe {
                _unsafe.to_tokens(&mut signature)
            }
            taget._fn.to_tokens(&mut signature);
            taget.name.to_tokens(&mut signature);
            taget.generics.to_tokens(&mut signature);

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

fn extract_attr_of_ident(target: &Ident, vec: &mut Vec<Attribute>) -> Option<Attribute> {
    for i in 0..vec.len() {
        if vec[i].path.get_ident() == Some(target) {
            return Some(vec.remove(i))
        }
    }
    None
}