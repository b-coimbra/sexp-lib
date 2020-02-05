use ext;
use lexer;

fn next_tokens(tokens: &lexer::TokenList, pos: usize) -> lexer::TokenList {
    tokens
        .into_iter()
        .filter(|&token| token.position > pos)
        .cloned()
        .collect()
}

// TODO: Create a container for all operations 
// Operation::sum(), Operation::div() etc
fn sum_operation(tokens: &lexer::TokenList, mut sum: u32) -> u32 {
    for token in tokens {
        match token.kind {
            lexer::TokenKind::Symbol(lexer::Symbol::RParen) => return sum,
            lexer::TokenKind::Type(lexer::Type::Int) => {
                sum += ext::to_u32(&token.value);

                return sum_operation(&next_tokens(tokens, token.position), sum);
            },
            _ => ()
        }
    }
    return sum;
}

pub fn parse(tokens: &lexer::TokenList, mut opened_parens: bool) -> () {
    if tokens.is_empty() { return }

    for token in tokens {
        match token.kind {
            lexer::TokenKind::Symbol(lexer::Symbol::LParen) => {
                opened_parens = true;

                return parse(&next_tokens(tokens, token.position), opened_parens);
            },
            lexer::TokenKind::Operator(lexer::Operator::Plus) => {
                // TODO: make nested expressions work by gathering the sub expressions and replacing them (evaluating)
                // to a new token with the value from the result
                println!("{}", sum_operation(&next_tokens(tokens, token.position), 0));
                return ();
            },
            lexer::TokenKind::Symbol(lexer::Symbol::RParen) => {
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
