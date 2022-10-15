use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, token::{Comma, Colon, Fn, Paren, Brace, Mut}, Type, parse::Parse, parenthesized, braced};


pub struct Return {
    pub fields: Punctuated<Field, Comma>
}
pub struct Function {
    _fn:      Fn,
    pub name: Ident,
    _paren:   Paren,
    pub args: Punctuated<Field, Comma>,
    // _arrow:   RArrow,
    // _mark:    Rem,
    _brace:   Brace,
    pub body: TokenStream,
}
pub struct Field {
    _mut:       Option<Mut>,  
    pub name:   Ident,
    _colon:     Colon,
    pub typexp: Type,
}
impl Parse for Return {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Field::parse)?
        })
    }
}
impl Parse for Function {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (args_buf, body_buf);
        Ok(Self {
            _fn:    input.parse()?,
            name:   input.parse()?,
            _paren: parenthesized!(args_buf in input),
            args:   args_buf.parse_terminated(Field::parse)?,
            // _arrow: input.parse()?,
            // _mark:   input.parse()?,
            _brace: braced!(body_buf in input),
            body:   {
                // let mut replcaed_body = TokenStream::new();

                body_buf.parse()?
                // body_buf.step(|corsor| {
                //     let mut rest = *corsor;
                //     while let Some((tt, next)) = rest.token_tree() {
                //         replcaed_body.extend(quote!(#tt));
                //     }
                //     return Ok(((), Cursor::empty()))
                //     
                // }).expect("failed to parse function");

                // replcaed_body
            }
/*
fn skip_past_next_at(input: ParseStream) -> Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '@' => {
                    return Ok(((), next));
                }
                _ => rest = next,
            }
        }
        Err(cursor.error("no `@` was found after this point"))
    })
}
*/
        })
    }
}
impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _mut:   input.parse()?,
            name:   input.parse()?,
            _colon: input.parse()?,
            typexp: input.parse()?,
        })
    }
}
