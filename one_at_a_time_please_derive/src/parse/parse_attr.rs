use ::proc_macro2::TokenStream;
use ::std::iter::Iterator;

use crate::ParseError;

pub fn parse_attr(attr: TokenStream) -> Result<(), ParseError> {
    let mut iter = attr.into_iter();

    match iter.next() {
        None => Ok(()),
        Some(_) => Err(ParseError::ContentFound),
    }
}
