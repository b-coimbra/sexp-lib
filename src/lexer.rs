use parser::{ Parser };

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
pub enum TokenType {
    Operator(Operator),
    Symbol(Symbol),
    Type(Type)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenType,
    pub position: usize
}

impl Token {
    fn new(value: String, kind: TokenType, position: usize) -> Self {
        Token { value, kind, position }
    }
}

pub type Tokens = Vec<Token>;

pub trait TokenStream {
    fn rest(&self, pos: usize) -> Tokens;
}

impl TokenStream for Tokens {
    fn rest(&self, pos: usize) -> Tokens {
        self
            .into_iter()
            .filter(|&token| token.position > pos)
            .cloned()
            .collect()
    }
}

pub fn tokenize(stream: String) -> Result<Tokens, ()> {
    let mut tokens: Tokens = Vec::new();

    let mut make_token = |value: String, kind: TokenType, pos: usize| -> () {
        tokens.push(Token::new(value, kind, pos));
    };

    for (pos, chars) in stream.chars().enumerate() {
        match chars {
            '(' => make_token("(".to_string(), TokenType::Symbol(Symbol::LParen), pos),
            ')' => make_token(")".to_string(), TokenType::Symbol(Symbol::RParen), pos),
            '+' => make_token("+".to_string(), TokenType::Operator(Operator::Plus), pos),
            other => {
                if other == ' ' { () }

                let num = other.to_digit(10);
                // let r = &stream[pos..(pos + 1)];

                match num {
                    Some(num) => {
                        // let mut counter = 1;
                        // let next_num: std::option::Option<u32> = Some(stream[pos..(pos + counter)].parse().unwrap());

                        // while let Some(next_number) = next_num {
                        //     println!("{}", next_number);
                        //     num += next_number;
                        //     counter += 1;
                        // }

                        make_token(num.to_string(), TokenType::Type(Type::Int), pos)
                    },
                    None => ()
                }
            }
        }
    }

    if tokens.len() == 0 { () }

    tokens.parse(false);

    Ok(tokens)
}
