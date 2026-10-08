use std::collections::binary_heap::Iter;

use crate::error::*;
use crate::tokenizer;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    StatementNode { node: StatementNode },
    LiteralNode { node: LiteralNode },
    OperatorNode { node: OperatorNode },
    VariableNode { node: VariableNode },
}

#[derive(Debug, Clone)]
pub enum ExpressionNodeStep {
    IfCondition,
    ReturnExpression,
    OperatorLeft,
    OperatorRight,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    If {
        condition: Box<ExpressionNode>,
        body: Vec<StatementNode>,
    },
    Return {
        expression: Box<ExpressionNode>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralNode {
    IntLiteral { number: i32 },
}

#[derive(Debug, Clone)]
pub struct OperatorNode {
    operator: Operator,
    left: Option<Box<ExpressionNode>>,
    right: Option<Box<ExpressionNode>>,
}

#[derive(Debug, Clone)]
pub struct VariableNode {
    name: String,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn node_at<'a>(
    root: &'a mut ExpressionNode,
    steps: &mut std::slice::Iter<'_, ExpressionNodeStep>,
) -> &'a mut ExpressionNode {
    match steps.next() {
        None => root,
        Some(step) => match step {
            ExpressionNodeStep::OperatorLeft => match root {
                ExpressionNode::OperatorNode { node } => {
                    node_at(node.left.as_deref_mut().unwrap(), steps)
                }
                _ => panic!("AAAAAAAAAA"),
            },
            ExpressionNodeStep::OperatorRight => match root {
                ExpressionNode::OperatorNode { node } => {
                    node_at(node.right.as_deref_mut().unwrap(), steps)
                }
                _ => panic!("AAAAAAAAAA"),
            },
            _ => panic!("AAAAAAAAAA"),
        },
    }
}

pub fn create_expression_untill(tokens: &Vec<tokenizer::Token>) -> Result<ExpressionNode, CompilerError> {
    let mut iter = tokens.iter();

    let mut stack: Vec<ExpressionNodeStep> = Vec::new();

    let mut root = match &(&mut iter).next().unwrap().token_type {
        tokenizer::TokenType::Literal { literal } => ExpressionNode::LiteralNode {
            node: token_literal_to_node(&literal),
        },
        _ => return Err(CompilerError::BuildingError),
    };

    for token in iter {
        let node = node_at(&mut root, &mut stack.iter());
        match node {
            ExpressionNode::VariableNode { node: _ } | ExpressionNode::LiteralNode { node: _ } => {
                match &token.token_type {
                    tokenizer::TokenType::Operator { operator } => {
                        *node = ExpressionNode::OperatorNode {
                            node: token_operator_to_node(
                                operator,
                                Some(Box::new((*node).clone())),
                                None,
                            ),
                        };
                    }
                    _ => panic!("ASDASDAS"),
                }
            }

            ExpressionNode::OperatorNode { node } => {
                if node.right.is_none() {
                    println!("adasd");
                    node.right = Some(Box::from(token_to_expression_node_type(&token.token_type)));
                    stack.push(ExpressionNodeStep::OperatorRight);
                } else {
                    panic!("ASDASDASDA");
                }
            }

            _ => panic!("UNKNOWN NODE!!!"),
        };
    }

    Ok(root)
}

fn token_literal_to_node(literal: &tokenizer::Literal) -> LiteralNode {
    match literal {
        tokenizer::Literal::IntLiteral { value } => LiteralNode::IntLiteral { number: *value },
    }
}

fn token_to_expression_node_type(token: &tokenizer::TokenType) -> ExpressionNode {
    match token {
        tokenizer::TokenType::Literal { literal } => match literal {
            tokenizer::Literal::IntLiteral { value } => ExpressionNode::LiteralNode {
                node: LiteralNode::IntLiteral { number: *value },
            },
        },
        tokenizer::TokenType::Identifier { identifier } => ExpressionNode::VariableNode {
            node: VariableNode {
                name: (*identifier).clone(),
            },
        },
        _ => panic!("NO TIMPLEMENTRADAS!!!"),
    }
}

fn token_operator_to_node(
    operator: &tokenizer::Operator,
    left: Option<Box<ExpressionNode>>,
    right: Option<Box<ExpressionNode>>,
) -> OperatorNode {
    match operator {
        tokenizer::Operator::Plus => OperatorNode {
            operator: Operator::Plus,
            left,
            right,
        },
        _ => panic!("AAAA"),
    }
}
