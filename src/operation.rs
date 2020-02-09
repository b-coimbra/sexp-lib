use lexer::*;
use ext;

pub trait Operations {
    fn sum(&self, sum: u32) -> u32;
}

impl Operations for Tokens {
    fn sum(&self, mut sum: u32) -> u32 {
        for token in self {
            match token.kind {
                TokenType::Symbol(Symbol::LParen) |
                TokenType::Operator(Operator::Plus) => {
                    return Operations::sum(&self.rest(token.position), sum);
                },
                TokenType::Type(Type::Int) => {
                    sum += ext::to_u32(&token.value);
                },
                _ => ()
            }
        }
        return sum;
    }
}
