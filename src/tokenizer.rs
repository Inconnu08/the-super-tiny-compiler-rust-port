#[derive(Debug, PartialEq)]
pub enum Token {
    ParenOpening,
    ParenClosing,
    Number(String),
    String(String),
    Name(String),
    Unknown,
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(ch) = char_iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '(' => tokens.push(Token::ParenOpening),
            ')' => tokens.push(Token::ParenClosing),
            '0'..='9' => {
                let mut value = String::new();
                value.push(ch);

                while let Some('0'..='9') = char_iter.peek() {
                    value.push(char_iter.next().unwrap());
                }

                tokens.push(Token::Number(value));
            }
            'a'..='z' => {
                let mut value = String::new();
                value.push(ch);

                while let Some('a'..='z') = char_iter.peek() {
                    value.push(char_iter.next().unwrap());
                }

                tokens.push(Token::Name(value));
            }
            '"' => {
                let mut value = String::new();

                while match char_iter.peek() {
                    Some('"') | None => false,
                    _ => true,
                } {
                    value.push(char_iter.next().unwrap());
                }

                if char_iter.peek() == Some(&'"') {
                    char_iter.next();
                }

                tokens.push(Token::String(value));
            }
            _ => return Err(format!("unknown character: {}", ch)),
        }
    }

    Ok(tokens)
}
