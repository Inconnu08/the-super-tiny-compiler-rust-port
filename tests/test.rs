use the_super_tiny_compiler_rust_port::tokenizer::{tokenizer, Token};

#[test]
fn tokenizer_works() {
    let input = "(add 2 (subtract 4 2))";

    let tokens = vec![
        Token::ParenOpening,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenOpening,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
        Token::ParenClosing,
    ];

    let tokenized_input = tokenizer(input);

    assert_eq!(tokens.len(), tokenized_input.len());
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(*token, tokenized_input[i]);
    }
}
