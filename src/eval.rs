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
