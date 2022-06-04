use ::proc_macro2::Delimiter;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;
use ::std::iter::Peekable;

use crate::Ast;
use crate::ParseError;

pub fn parse_function(func: TokenStream) -> Result<Ast, ParseError> {
    let mut iter = func.into_iter().peekable();

    let function_definition = chomp_function_definition(&mut iter)?;
    let function_body = chomp_function_body(&mut iter)?;

    if iter.next().is_some() {
        return Err(ParseError::ExtraContentFound);
    }

    Ok(Ast {
        function_definition,
        function_body,
    })
}

fn chomp_function_definition<I>(iter: &mut Peekable<I>) -> Result<TokenStream, ParseError>
where
    I: Iterator<Item = TokenTree>,
{
    let mut function_definition = vec![];

    while let Some(token) = iter.peek() {
        if is_code_block(token) {
            return Ok(function_definition.into_iter().collect());
        }

        function_definition.push(iter.next().unwrap());
    }

    Err(ParseError::FunctionDefinitionExpected)
}

fn chomp_function_body<I>(iter: &mut I) -> Result<TokenStream, ParseError>
where
    I: Iterator<Item = TokenTree>,
{
    match iter.next() {
        Some(token_tree) => match token_tree {
            TokenTree::Group(group) => return Ok(group.stream()),
            _ => {}
        },
        _ => {}
    }

    Err(ParseError::FunctionBodyExpected)
}

fn is_code_block(token_tree: &TokenTree) -> bool {
    match token_tree {
        TokenTree::Group(group) => return group.delimiter() == Delimiter::Brace,
        _ => false,
    }
}
