use crate::error::*;
use crate::tokenizer;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    StatementNode { node: StatementNode },
    LiteralNode { node: LiteralNode },
    OperatorNode { node: OperatorNode },
    VariableNode { node: VariableNode },
    Empty,
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

pub fn create_expression(tokens: &Vec<tokenizer::Token>) -> Result<ExpressionNode, CompilerError> {
    let mut stack: Vec<ExpressionNode> = Vec::new();
    let mut working_node: Option<ExpressionNode> = None;

    for token in tokens.iter() {
        working_node = match &mut working_node {
            None => {
                // no working node, set first thingamabove
                Some(match &token.token_type {
                    tokenizer::TokenType::Literal { literal } => ExpressionNode {
                        parent: None,
                        node: ExpressionNode::LiteralNode {
                            node: token_literal_to_node(&literal),
                        },
                    },
                    _ => return Err(CompilerError::BuildingError),
                })
            }
            Some(node) => Some(step(node, token)?),
        }
    }

    working_node.ok_or(CompilerError::BuildingError)
}

fn step(
    working_node: &mut ExpressionNode,
    token: &tokenizer::Token,
) -> Result<Box<ExpressionNode>, CompilerError> {
    match working_node.node {
        ExpressionNode::VariableNode { node: _ } | ExpressionNode::LiteralNode { node: _ } => {
            match &token.token_type {
                // while wokring is literal, make the literal child of new operator
                tokenizer::TokenType::Operator { operator } => Ok(Box::from(ExpressionNode {
                    parent: working_node.parent,
                    node: ExpressionNode::OperatorNode {
                        node: token_operator_to_node(
                            operator,
                            Some(Box::from((*working_node).clone())),
                            None,
                        ),
                    },
                })),
                _ => return Err(CompilerError::BuildingError),
            }
        }
        ExpressionNode::OperatorNode { node: node } => {
            if node.right.is_none() {
                let mut n = Box::from(ExpressionNode {
                    parent: working_node.parent.clone(),
                    node: ExpressionNode::Empty,
                });

                n.node = ExpressionNode::OperatorNode {
                    node: OperatorNode {
                        operator: node.operator,
                        left: node.left,
                        right: Some(Box::from(token_to_expression_node(
                            &token.token_type,
                            Some(n),
                        ))),
                    },
                };

                Ok(n)
            } else {
                panic!("AAAAAAa");
            }
        }

        _ => panic!("UNKNOWN NODE!!!"),
    }
}

fn token_literal_to_node(literal: &tokenizer::Literal) -> LiteralNode {
    match literal {
        tokenizer::Literal::IntLiteral { value } => LiteralNode::IntLiteral { number: *value },
    }
}

fn token_to_expression_node(
    token: &tokenizer::TokenType,
    parent: Option<Box<ExpressionNode>>,
) -> ExpressionNode {
    ExpressionNode {
        parent: parent,
        node: token_to_expression_node_type(token),
    }
}

fn token_to_expression_node_type(token: &tokenizer::TokenType) -> ExpressionNode {
    match token {
        tokenizer::TokenType::Literal { literal } => match literal {
            tokenizer::Literal::IntLiteral { value } => ExpressionNode::LiteralNode {
                node: LiteralNode::IntLiteral { number: *value },
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
