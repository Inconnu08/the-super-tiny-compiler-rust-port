use crate::tokenizer::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum NodeType {
    Program,
    CallExpression,
    StringLiteral,
    NumberLiteral,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Program {
        body: Vec<Node>,
    },
    CallExpression {
        identifier: String,
        params: Vec<Node>,
    },
    StringLiteral(String),
    NumberLiteral(String),
}

impl Node {
    fn get_type(&self) -> NodeType {
        match *self {
            Node::Program { .. } => NodeType::Program,
            Node::CallExpression { .. } => NodeType::CallExpression,
            Node::StringLiteral(_) => NodeType::StringLiteral,
            Node::NumberLiteral(_) => NodeType::NumberLiteral,
        }
    }
}

pub struct Visitor {
    pub enter: Option<Box<dyn Fn(&Node, Option<&Node>)>>,
    pub exit: Option<Box<dyn Fn(&Node, Option<&Node>)>>,
}

pub fn parser(tokens: Vec<Token>) -> Result<Node, String> {
    fn walk(token: Token, token_iter: &mut Peekable<IntoIter<Token>>) -> Result<Node, String> {
        match token {
            Token::Number(value) => Ok(Node::NumberLiteral(value.to_string())),
            Token::String(value) => Ok(Node::StringLiteral(value.to_string())),
            Token::ParenOpening => {
                if let Some(token) = token_iter.next() {
                    match token {
                        Token::Name(identifier) => {
                            let mut params: Vec<Node> = vec![];

                            while match token_iter.peek() {
                                Some(Token::ParenClosing) | None => false,
                                _ => true,
                            } {
                                match walk(token_iter.next().unwrap(), token_iter) {
                                    Ok(nodes) => params.push(nodes),
                                    Err(value) => return Err(value),
                                }
                            }

                            token_iter.next().unwrap(); // skip closing paren token
                            Ok(Node::CallExpression { identifier, params })
                        }
                        _ => {
                            return Err(format!(
                                "{:?} isn't followed by a {:?}.",
                                Token::ParenOpening,
                                Token::Name("example".to_string())
                            ))
                        }
                    }
                } else {
                    return Err(format!(
                        "{:?} isn't followed by a node.",
                        Token::ParenOpening
                    ));
                }
            }
            _ => return Err(format!("I dont know what this token is: {:?}", token)),
        }
    }

    let mut body: Vec<Node> = vec![];

    let mut token_iter = tokens.into_iter().peekable();
    while let Some(token) = token_iter.next() {
        match walk(token, &mut token_iter) {
            Ok(nodes) => body.push(nodes),
            Err(value) => return Err(value),
        }
    }

    Ok(Node::Program { body })
}

pub fn traverser(node: Node, visitors: HashMap<NodeType, Visitor>) {
    fn traverse_nodes(
        nodes: &Vec<Node>,
        parent: Option<&Node>,
        visitors: &HashMap<NodeType, Visitor>,
    ) {
        for node in nodes {
            traverse_node(&node, parent, visitors);
        }
    }

    fn traverse_node(node: &Node, parent: Option<&Node>, visitors: &HashMap<NodeType, Visitor>) {
        let visitor = visitors.get(&node.get_type());

        if visitor.is_some() {
            if let Some(ref enter) = visitor.unwrap().enter {
                enter(&node, parent);
            }
        }

        match *node {
            Node::Program { ref body } => traverse_nodes(body, Some(node), visitors),
            Node::CallExpression { ref params, .. } => traverse_nodes(params, Some(node), visitors),
            _ => {} //_ => println!("We can't have an unknown node here!"),
        }

        if visitor.is_some() {
            if let Some(ref exit) = visitor.unwrap().exit {
                exit(&node, parent);
            }
        }
    }

    traverse_node(&node, None, &visitors);
}
