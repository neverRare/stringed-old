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
    pub fn describe(&self) -> &str {
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
pub fn lex(src: &str) -> Result<Vec<Token>, &str> {
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
    }    if inside_str {
        Err("unterminated string")
    } else {
        Ok(vec)
    }
}
