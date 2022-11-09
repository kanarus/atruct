use quote::quote;
use crate::internals::Build;
use super::interpreter::{ReturnStreams, WithReturnStreams};


impl Build for WithReturnStreams {
    fn build(self) -> proc_macro2::TokenStream {
        let WithReturnStreams {
            impl_block_stream,
            structs_stream
        } = self;

        quote!(
            #structs_stream
            #impl_block_stream
        )
    }
}


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