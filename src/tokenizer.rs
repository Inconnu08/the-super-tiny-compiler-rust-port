#[derive(Debug, PartialEq)]
pub enum Token {
    ParenOpening,
    ParenClosing,
    Number(String),
    String(String),
    Name(String),
    Unknown,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(mut ch) = char_iter.next() {
        match ch {
            ch if ch.is_whitespace() => {}
            '(' => tokens.push(Token::ParenOpening),
            ')' => tokens.push(Token::ParenClosing),
            '0'..='9' => {
                let mut value = String::new();
                value.push(ch);

                while match char_iter.peek() {
                    Some('0'..='9') => true,
                    _ => false,
                } {
                    value.push(ch);
                    ch = match char_iter.next() {
                        Some(ch) => ch,
                        None => break,
                    }
                }
                tokens.push(Token::Number(value));
            }
            _ => tokens.push(Token::Unknown),
        }
    }

    tokens
}
