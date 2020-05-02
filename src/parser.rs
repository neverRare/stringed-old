use super::error_msg::expect;
use super::lexer::{lex, Token};
type BoxExpr<'a> = Box<Expr<'a>>;
pub enum Expr<'a> {
    Group(BoxExpr<'a>),
    Input,
    String(&'a str),
    Concat(BoxExpr<'a>, BoxExpr<'a>),
    Slice {
        expr: BoxExpr<'a>,
        lower: Option<BoxExpr<'a>>,
        upper: Option<BoxExpr<'a>>,
    },
    Eval(BoxExpr<'a>),
}
pub struct FullExpr<'a> {
    pub expr: Expr<'a>,
    token_count: usize,
    pub have_input: bool,
}
fn get_optional_delimited_expr<'a>(
    tokens: &[Token<'a>],
    assert_start: &Token,
    end: &Token,
) -> Result<Option<FullExpr<'a>>, String> {
    assert_eq!(&tokens[0], assert_start);
    let rest = &tokens[1..];
    if rest.is_empty() {
        Err(expect(end.describe(), "EOF"))
    } else if &rest[0] == end {
        Ok(None)
    } else {
        let operand = get_expr(rest)?;
        let last_token = &tokens[operand.token_count + 1];
        if last_token == end {
            Ok(Some(operand))
        } else {
            Err(expect(end.describe(), last_token.describe()))
        }
    }
}
fn get_delimited_expr<'a>(
    tokens: &[Token<'a>],
    assert_start: &Token,
    end: &Token,
) -> Result<FullExpr<'a>, String> {
    match get_optional_delimited_expr(tokens, assert_start, end)? {
        Some(expr) => Ok(expr),
        None => Err(expect("expression", end.describe())),
    }
}
fn get_single_expr<'a>(tokens: &[Token<'a>]) -> Result<FullExpr<'a>, String> {
    match &tokens[0] {
        Token::Input => Ok(FullExpr {
            token_count: 1,
            expr: Expr::Input,
            have_input: true,
        }),
        Token::String(string) => Ok(FullExpr {
            token_count: 1,
            expr: Expr::String(string),
            have_input: false,
        }),
        Token::Dollar => {
            let full_expr = get_expr(&tokens[1..])?;
            Ok(FullExpr {
                expr: Expr::Eval(Box::new(full_expr.expr)),
                token_count: full_expr.token_count + 1,
                have_input: full_expr.have_input,
            })
        }
        Token::OpenParen => {
            let full_expr = get_delimited_expr(tokens, &Token::OpenParen, &Token::CloseParen)?;
            Ok(FullExpr {
                token_count: full_expr.token_count + 2,
                have_input: full_expr.have_input,
                expr: Expr::Group(Box::new(full_expr.expr)),
            })
        }
        token => Err(expect("expression", token.describe())),
    }
}
fn get_expr<'a>(tokens: &[Token<'a>]) -> Result<FullExpr<'a>, String> {
    get_expr_from(tokens, get_single_expr(tokens)?)
}
fn get_expr_from<'a>(
    tokens: &[Token<'a>],
    last_expr: FullExpr<'a>,
) -> Result<FullExpr<'a>, String> {
    let rest_tokens = &tokens[last_expr.token_count..];
    if rest_tokens.is_empty() {
        Ok(last_expr)
    } else {
        match rest_tokens[0] {
            Token::Plus => {
                let other_expr = get_expr(&rest_tokens[1..])?;
                get_expr_from(
                    tokens,
                    FullExpr {
                        expr: Expr::Concat(Box::new(last_expr.expr), Box::new(other_expr.expr)),
                        have_input: last_expr.have_input || other_expr.have_input,
                        token_count: last_expr.token_count + other_expr.token_count + 1,
                    },
                )
            }
            Token::OpenBracket => get_expr_from(tokens, get_slice(last_expr, rest_tokens)?),
            _ => Ok(last_expr),
        }
    }
}
fn get_slice<'a>(
    last_expr: FullExpr<'a>,
    rest_tokens: &[Token<'a>],
) -> Result<FullExpr<'a>, String> {
    let lower_expr = get_optional_delimited_expr(rest_tokens, &Token::OpenBracket, &Token::Colon)?;
    let upper_rest_tokens = match &lower_expr {
        Some(full_expr) => &rest_tokens[full_expr.token_count + 1..],
        None => &rest_tokens[1..],
    };
    let upper_expr =
        get_optional_delimited_expr(upper_rest_tokens, &Token::Colon, &Token::CloseBracket)?;
    Ok(FullExpr {
        have_input: last_expr.have_input
            || match &lower_expr {
                Some(expr) => expr.have_input,
                None => false,
            }
            || match &upper_expr {
                Some(expr) => expr.have_input,
                None => false,
            },
        token_count: last_expr.token_count
            + match &lower_expr {
                Some(expr) => expr.token_count,
                None => 0,
            }
            + match &upper_expr {
                Some(expr) => expr.token_count,
                None => 0,
            }
            + 3,
        expr: Expr::Slice {
            expr: Box::new(last_expr.expr),
            lower: match lower_expr {
                Some(expr) => Some(Box::new(expr.expr)),
                None => None,
            },
            upper: match upper_expr {
                Some(expr) => Some(Box::new(expr.expr)),
                None => None,
            },
        },
    })
}
pub fn parse(src: &str) -> Result<FullExpr, String> {
    let token = lex(src)?;
    if token.is_empty() {
        Err("invalid expression, it can't be empty".to_string())
    } else {
        let expr = get_expr(&token)?;
        if expr.token_count == token.len() {
            Ok(expr)
        } else {
            Err(format!("unexpected {}", token[expr.token_count].describe()))
        }
    }
}
