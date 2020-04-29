mod lexer {
    #[derive(PartialEq, Debug)]
    pub enum Token<'a> {
        Input,
        String(&'a str),
        Dollar,
        Plus,
        Colon,
        OpenParen,
        CloseParen,
        OpenBracket,
        CloseBracket,
    }
    impl<'a> Token<'a> {
        pub fn describe(&self) -> &'static str {
            use Token::*;
            match self {
                Input => "_",
                String(_) => "string",
                Dollar => "$",
                Plus => "+",
                Colon => ":",
                OpenParen => "`(`",
                CloseParen => "`)`",
                OpenBracket => "`[`",
                CloseBracket => "`]`",
            }
        }
    }
    // perhaps this could an iterator instead, or maybe not
    pub fn lex(src: &str) -> Result<Vec<Token>, &'static str> {
        let mut vec = Vec::new();
        let mut inside_str = false;
        let mut is_escaping = false;
        let mut str_start = 0usize;
        for i in 0..src.len() {
            let elem = &src[i..i + 1];
            if inside_str {
                if is_escaping {
                    is_escaping = false;
                } else if elem == "\\" {
                    is_escaping = true;
                } else if elem == "\"" {
                    inside_str = false;
                    assert_ne!(str_start, 0);
                    vec.push(Token::String(&src[str_start..i]));
                }
                continue;
            }
            if elem == "\"" {
                inside_str = true;
                str_start = i + 1;
                continue;
            }
            let token = match elem {
                "_" => Token::Input,
                "$" => Token::Dollar,
                "+" => Token::Plus,
                ":" => Token::Colon,
                "(" => Token::OpenParen,
                ")" => Token::CloseParen,
                "[" => Token::OpenBracket,
                "]" => Token::CloseBracket,
                _ => continue,
            };
            vec.push(token);
        }
        if inside_str {
            Err("unterminated string")
        } else {
            Ok(vec)
        }
    }
}
mod error_msg {
    pub fn expect(expecting: &str, got: &str) -> String {
        format!("expecting {}, got {}", expecting, got)
    }
}
mod parser {
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
    fn get_from_group<'a>(tokens: &[Token<'a>]) -> Result<FullExpr<'a>, String> {
        assert_eq!(tokens[0], Token::OpenParen);
        let operand = get_expr(&tokens[1..])?;
        if tokens.len() < operand.token_count {
            Err(expect("`)`", "EOF"))
        } else {
            match &tokens[operand.token_count + 1] {
                Token::CloseParen => Ok(FullExpr {
                    expr: Expr::Group(Box::new(operand.expr)),
                    token_count: operand.token_count + 2,
                    have_input: operand.have_input,
                }),
                token => Err(expect("`)`", token.describe())),
            }
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
            Token::OpenParen => get_from_group(tokens),
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
    // TODO: clean this mess
    fn get_slice<'a>(
        last_expr: FullExpr<'a>,
        rest_tokens: &[Token<'a>],
    ) -> Result<FullExpr<'a>, String> {
        assert_eq!(rest_tokens[0], Token::OpenBracket);
        let lower_rest_tokens = &rest_tokens[1..];
        let lower_expr = if lower_rest_tokens.is_empty() {
            return Err(expect("expression or :", "EOF"));
        } else if let Token::Colon = lower_rest_tokens[0] {
            None
        } else {
            Some(get_expr(lower_rest_tokens)?)
        };
        let upper_rest_tokens = match &lower_expr {
            Some(full_expr) => {
                let rest_tokens = &lower_rest_tokens[full_expr.token_count..];
                if rest_tokens.is_empty() {
                    return Err(expect(":", "EOF"));
                }
                match &rest_tokens[0] {
                    Token::Colon => &rest_tokens[1..],
                    token => return Err(expect(":", token.describe())),
                }
            }
            None => &lower_rest_tokens[1..],
        };
        let upper_expr = if upper_rest_tokens.is_empty() {
            return Err(expect("expression or `]`", "EOF"));
        } else if let Token::CloseBracket = upper_rest_tokens[0] {
            None
        } else {
            Some(get_expr(upper_rest_tokens)?)
        };
        if let Some(full_expr) = &upper_expr {
            let rest_tokens = &upper_rest_tokens[full_expr.token_count..];
            if rest_tokens.is_empty() {
                return Err(expect("]", "EOF"));
            }
            let next_token = &rest_tokens[0];
            if let Token::CloseBracket = next_token {
            } else {
                return Err(expect("]", next_token.describe()));
            }
        }
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
}
mod eval {
    use super::parser::{parse, Expr};
    fn unescape(src: &str) -> String {
        let mut result = String::new();
        let mut is_escaping = false;
        for i in 0..src.len() {
            let elem = &src[i..i + 1];
            if is_escaping {
                is_escaping = false;
                result.push_str(elem);
                continue;
            }
            if elem == "\\" {
                is_escaping = true;
            } else {
                result.push_str(elem);
            }
        }
        result
    }
    fn parse_number(src: &str) -> Result<usize, String> {
        match src.parse() {
            Ok(num) => Ok(num),
            Err(err) => Err(format!("{}", err)),
        }
    }
    pub fn eval_expr(expr: &Expr, input: &str) -> Result<String, String> {
        match expr {
            Expr::String(content) => Ok(unescape(content)),
            Expr::Group(x) => Ok(eval_expr(&*x, input)?),
            Expr::Input => Ok(input.to_string()),
            Expr::Concat(a, b) => Ok(eval_expr(&*a, input)? + &eval_expr(&*b, input)?),
            Expr::Slice { expr, upper, lower } => {
                let expr = eval_expr(&*expr, input)?;
                let lower: usize = match lower {
                    Some(expr) => parse_number(&eval_expr(&*expr, input)?)?,
                    None => 0,
                };
                let upper: usize = match upper {
                    Some(expr) => parse_number(&eval_expr(&*expr, input)?)?,
                    None => expr.len(),
                };
                if lower > upper || upper > expr.len() {
                    Err("out of bound".to_string())
                } else {
                    Ok(expr[lower..upper].to_string())
                }
            }
            Expr::Eval(expr) => eval(&eval_expr(&*expr, input)?, input),
        }
    }
    pub fn eval(src: &str, input: &str) -> Result<String, String> {
        eval_expr(&parse(src)?.expr, input)
    }
}
pub use eval::{eval, eval_expr};
pub use parser::parse;
