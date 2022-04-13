#[derive(Debug, PartialEq)]
pub enum Token {
    ParenOpening,
    ParenClosing,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for char in input.chars() {
        match char {
            '(' => tokens.push(Token::ParenOpening),
            _ => tokens.push(Token::ParenClosing),
        }
    }

    tokens
}
