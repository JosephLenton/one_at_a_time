use ::proc_macro2::TokenStream;
use ::quote::quote;

use crate::Ast;

pub fn output(ast: Ast) -> TokenStream {
    let function_definition = &ast.function_definition;
    let function_body = &ast.function_body;

    quote! {
        #function_definition {
          let __one_at_a_time_global_lock_guard__ = ::one_at_a_time_please::ONE_AT_A_TIME_GLOBAL_LOCK.lock();
          let r = {
            #function_body
          };
          ::std::mem::drop(__one_at_a_time_global_lock_guard__);

          r
        }
    }
}
