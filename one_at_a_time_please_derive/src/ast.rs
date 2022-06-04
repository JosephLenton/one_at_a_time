use ::proc_macro2::TokenStream;

use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

pub struct Ast {
    pub function_definition: TokenStream,
    pub function_body: TokenStream,
}

impl Debug for Ast {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(
            fmt,
            "Ast {{
      function_definition: {{{}}},
      function_body: {{{}}},
    }}",
            self.function_definition.to_string(),
            self.function_body.to_string()
        )
    }
}

impl Display for Ast {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Debug::fmt(self, fmt)
    }
}

impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
