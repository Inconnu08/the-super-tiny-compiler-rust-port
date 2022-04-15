use the_super_tiny_compiler_rust_port::tokenizer::{tokenizer, Token};

#[test]
fn tokenizer_works() {
    let input = "(add 22 \"ff\" (subtract 4 2))";

    let expected_tokens = vec![
        Token::ParenOpening,
        Token::Name("add".to_string()),
        Token::Number("22".to_string()),
        Token::String("ff".to_string()),
        Token::ParenOpening,
        Token::Name("subtract".to_string()),
        Token::Number("4".to_string()),
        Token::Number("2".to_string()),
        Token::ParenClosing,
        Token::ParenClosing,
    ];

    assert_eq!(Ok(expected_tokens), tokenizer(input));
}
