use ::proc_macro::TokenStream;

mod parse;
pub(crate) use crate::parse::*;

mod output;
pub(crate) use crate::output::*;

mod ast;
pub(crate) use crate::ast::*;

/// Marks this function to only run, from a single thread at a time.
/// If two threads call the function at the same time,
/// one will be blocked until the other is completed.
///
/// Note they will _also_ be blocked against _all_ functions which are
/// marked with `#[one_at_a_time]`.
#[proc_macro_attribute]
pub fn one_at_a_time(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse(attr.into(), item.into()) {
        Ok(ast) => output(ast).into(),
        Err(err) => panic!("{}", err),
    }
}
