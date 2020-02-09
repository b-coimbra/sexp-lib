use lexer::*;
use operation::{ Operations };

pub trait Parser {
    fn parse(&self, opened_parens: bool) -> ();
}

impl Parser for Tokens {
    fn parse(&self, mut opened_parens: bool) -> () {
        if self.is_empty() { return }

        for token in self {
            match token.kind {
                TokenType::Symbol(Symbol::LParen) => {
                    opened_parens = true;

                    return Parser::parse(&self.rest(token.position), opened_parens);
                },
                TokenType::Operator(Operator::Plus) => {
                    // TODO: gather the sub expressions by replacing them (evaluating)
                    // to a new token with the value from the result
                    println!("{}", Operations::sum(&self.rest(token.position), 0));
                    return ();
                },
                TokenType::Symbol(Symbol::RParen) => {
                    if opened_parens {
                        println!("Finished parsing the tokens.");
                        return ();
                    }
                    else {
                        // TODO: add line position
                        println!("Parens mismatch in column: {}", token.position + 1);
                    }
                },
                _ => ()
            }
        }
    }
}
