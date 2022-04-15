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

    while let Some(ch) = char_iter.next() {
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
                    if let Some(ch) = char_iter.next() {
                        value.push(ch);
                    }
                }
                tokens.push(Token::Number(value));
            }
            'a'..='z' => {
                let mut value = String::new();
                value.push(ch);

                while match char_iter.peek() {
                    Some('a'..='z') => true,
                    _ => false,
                } {
                    if let Some(ch) = char_iter.next() {
                        value.push(ch);
                    }
                }

                tokens.push(Token::Name(value));
            }
            '"' => {
                let mut value = String::new();

                while match char_iter.peek() {
                    Some('"') | None => false,
                    _ => true,
                } {
                    if let Some(ch) = char_iter.next() {
                        value.push(ch);
                    }
                }

                if char_iter.peek() == Some(&'"') {
                    char_iter.next();
                }

                tokens.push(Token::String(value));
            }
            _ => tokens.push(Token::Unknown),
        }
    }

    tokens
}
