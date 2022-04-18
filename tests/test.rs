use std::collections::HashMap;
use the_super_tiny_compiler_rust_port::ast::{parser, traverser, Node, NodeType, Visitor};
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

#[test]
fn parser_works() {
    let expected_ast = Node::Program {
        body: vec![Node::CallExpression {
            identifier: "add".to_string(),
            params: vec![
                Node::NumberLiteral("22".to_string()),
                Node::CallExpression {
                    identifier: "subtract".to_string(),
                    params: vec![
                        Node::NumberLiteral("4".to_string()),
                        Node::NumberLiteral("2".to_string()),
                    ],
                },
            ],
        }],
    };

    let tokens = vec![
        Token::ParenOpening,
        Token::Name("add".to_string()),
        Token::Number("22".to_string()),
        Token::ParenOpening,
        Token::Name("subtract".to_string()),
        Token::Number("4".to_string()),
        Token::Number("2".to_string()),
        Token::ParenClosing,
        Token::ParenClosing,
    ];

    assert_eq!(Ok(expected_ast), parser(tokens.clone()));

    let mut visitors = HashMap::new();
    visitors.insert(
        NodeType::Program,
        Visitor {
            enter: Some(Box::new(|node: &Node, parent: Option<&Node>| {
                println!("test enter works!")
            })),
            exit: Some(Box::new(|node: &Node, parent: Option<&Node>| {
                println!("test exit works!")
            })), // exit: None,
        },
    );
    traverser(parser(tokens.clone()).unwrap(), visitors);
}
