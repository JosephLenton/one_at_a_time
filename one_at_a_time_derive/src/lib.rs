use ::proc_macro::TokenStream;

mod parse;
pub(crate) use crate::parse::*;

mod output;
pub(crate) use crate::output::*;

mod ast;
pub(crate) use crate::ast::*;

#[proc_macro_attribute]
pub fn one_at_a_time(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse(attr.into(), item.into()) {
        Ok(ast) => output(ast).into(),
        Err(err) => panic!("{}", err),
    }
}
