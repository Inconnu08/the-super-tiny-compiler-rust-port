use the_super_tiny_compiler_rust_port::tokenizer::{tokenizer, Token};

#[test]
fn tokenizer_works() {
    let input = "(add 2 (subtract 4 2))";

    let tokens = vec![
        Token::ParenOpening,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Number("2".to_string()),
        Token::ParenOpening,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Unknown,
        Token::Number("4".to_string()),
        Token::Number("2".to_string()),
        Token::ParenClosing,
        Token::ParenClosing,
    ];

    assert_eq!(tokens, tokenizer(input));
}
