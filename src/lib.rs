mod error_msg;
mod eval;
mod lexer;
mod parser;
pub use eval::{eval, eval_expr};
pub use parser::parse;
