use quote::quote;

use crate::internals::Build;

use super::interpreter::ReturnStreams;

impl Build for ReturnStreams {
    fn build(self) -> proc_macro2::TokenStream {
        let ReturnStreams{
            struct_stream,
            function_stream
        } = self;
        
        quote!(
            #struct_stream
            #function_stream
        )
    }
}