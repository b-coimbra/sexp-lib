use parser;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Div,
    Times
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Int,
    Bool
}

#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    LParen,
    RParen
}

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    Operator(Operator),
    Symbol(Symbol),
    Type(Type)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub position: usize
}

pub type TokenList = Vec<Token>;

impl Token {
    fn new(value: String, kind: TokenKind, position: usize) -> Self {
        Token { value, kind, position }
    }
}

pub fn tokenize(stream: String) -> Result<TokenList, ()> {
    let mut tokens: TokenList = Vec::new();

    let mut make_token = |value: String, kind: TokenKind, pos: usize| -> () {
        tokens.push(Token::new(value, kind, pos));
    };

    for (pos, chars) in stream.chars().enumerate() {
        match chars {
            '(' => make_token("(".to_string(), TokenKind::Symbol(Symbol::LParen), pos),
            ')' => make_token(")".to_string(), TokenKind::Symbol(Symbol::RParen), pos),
            '+' => make_token("+".to_string(), TokenKind::Operator(Operator::Plus), pos),
            other => {
                if other == ' ' { () }

                let num = other.to_digit(10);

                match num {
                    Some(num) => {
                        make_token(num.to_string(), TokenKind::Type(Type::Int), pos)
                    },
                    None => ()
                }
            }
        }
    }

    if tokens.len() == 0 { () }

    parser::parse(&tokens, false);

    Ok(tokens)
}
