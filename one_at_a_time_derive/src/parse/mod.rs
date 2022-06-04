mod parse_error;
pub use self::parse_error::*;

mod parse_attr;
pub use self::parse_attr::*;

mod parse_function;
pub use self::parse_function::*;

use crate::Ast;
use ::proc_macro2::TokenStream;

pub fn parse(attr: TokenStream, func: TokenStream) -> Result<Ast, ParseError> {
    parse_attr(attr)?;
    parse_function(func)
}

#[cfg(test)]
mod describe_impl {
    use super::*;
    use ::pretty_assertions::assert_eq;
    use ::quote::quote;

    #[test]
    fn it_should_parse_functions_correctly() {
        let output = parse(
            quote! {},
            quote! {
              pub fn foo_blah<I>(a: u32, b: String)
                where
                  I : Iterator<Item = Something>,
              {
                a + b
              }
            },
        );

        let expected = Ast {
            function_definition: quote! {
              pub fn foo_blah<I>(a: u32, b: String)
              where
                I : Iterator<Item = Something>,
            },
            function_body: quote! {
              a + b
            },
        };

        assert_eq!(output, Ok(expected));
    }
}
