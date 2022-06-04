use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ContentFound,
    FunctionDefinitionExpected,
    FunctionBodyExpected,
    ExtraContentFound,
}

impl ParseError {
    pub fn error_message(&self) -> &'static str {
        match self {
      Self::ContentFound => "Attributes found. Expected no extra attributes provided.",
      Self::FunctionDefinitionExpected => "Function definition not found. Expected function definition, i.e. `pub fn foobar()`",
      Self::FunctionBodyExpected => "Function body not found. Expected a function body, i.e. `{ ... }`",
      Self::ExtraContentFound => "Extra content found after function. Expected no more tokens after the function it's self.",
    }
    }
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.error_message())
    }
}
