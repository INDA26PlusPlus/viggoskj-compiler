use core::num;
use std::any::Any;
use std::iter::Peekable;
use std::panic::PanicHookInfo;

use crate::ast::ExpressionNodeStep::ReturnExpression;
use crate::error::*;
use crate::tokenizer;
use crate::tokenizer::EnclosingType;
use crate::tokenizer::FlowControllType;
use crate::tokenizer::SyntaxFlowType;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    StatementNode { node: StatementNode },
    LiteralNode { node: LiteralNode },
    OperatorNode { node: OperatorNode },
    VariableNode { node: VariableNode },
    FunctionCall { node: FunctionCallNode },
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
        body: Vec<ExpressionNode>,
    },
    Return {
        expression: Option<Box<ExpressionNode>>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralNode {
    IntLiteral { number: i32 },
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    identifier: String,
    arguments: Vec<ExpressionNode>,
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
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct FunctionDeclarationNode {
    identifier: String,
    body: Vec<ExpressionNode>,
}

#[derive(Debug)]
pub struct Program {
    functions: Vec<FunctionDeclarationNode>,
}

pub fn create_program(tokens: Vec<tokenizer::Token>) -> Result<Program, CompilerError> {
    let mut prog = Program {
        functions: Vec::new(),
    };

    let mut iter = tokens.iter().peekable();

    while iter.peek().is_some() {
        prog.functions.push(create_function_declaration(&mut iter)?);
    }

    Ok(prog)
}

fn create_function_declaration(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
) -> Result<FunctionDeclarationNode, CompilerError> {
    let return_type_token = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "invalid return type".to_string(),
        })?;
    let function_name_token = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "invalid name".to_string(),
        })?;
    let _ = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "".to_string(),
        })?;

    while let arg_type = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "function declaraiton end to soon".to_string(),
        })?
        && arg_type.token_type
            != (tokenizer::TokenType::Enclosing {
                enclosing_type: tokenizer::EnclosingType::Bracket,
                open: false,
            })
    {}

    let mut expressions = create_block(tokens)?;

    Ok(FunctionDeclarationNode {
        identifier: match &function_name_token.token_type {
            tokenizer::TokenType::Identifier { identifier } => identifier.clone(),
            _ => {
                return Err(CompilerError::InvalidFunctionDeclaration {
                    reason: "invalid name token: ".to_string(),
                });
            }
        },
        body: expressions,
    })
}

fn create_function_call(
    identifier: String,
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
) -> Result<ExpressionNode, CompilerError> {
    let _ = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "".to_string(),
        })?;

    let mut arguments: Vec<ExpressionNode> = Vec::new();

    while let next = tokens
        .peek()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "function declaraiton end to soon".to_string(),
        })?
        && next.token_type
            != (tokenizer::TokenType::Enclosing {
                enclosing_type: tokenizer::EnclosingType::Bracket,
                open: false,
            })
    {
        println!("{:?}", next.token_type);
        arguments.push(create_expression_until_stop(tokens)?);
    }

    tokens.next();

    Ok(ExpressionNode::FunctionCall {
        node: FunctionCallNode {
            arguments,
            identifier,
        },
    })
}

fn create_expression_until_stop(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
) -> Result<ExpressionNode, CompilerError> {
    let mut stack: Vec<ExpressionNodeStep> = Vec::new();

    let mut root = tokens_to_expression_node_type(tokens)?;
    while let Some(token) = tokens.peek() {
        match token.token_type {
            tokenizer::TokenType::Enclosing {
                enclosing_type: EnclosingType::Bracket,
                open: false,
            }
            | tokenizer::TokenType::SyntaxFlow {
                syntax_flow_type: tokenizer::SyntaxFlowType::EndLine,
            }
            | tokenizer::TokenType::SyntaxFlow {
                syntax_flow_type: tokenizer::SyntaxFlowType::Comma,
            } => {
                println!("Expression created");
                return Ok(root);
            }
            _ => {}
        }

        let node = node_at(&mut root, &mut stack.iter());
        match node {
            ExpressionNode::VariableNode { node: _ }
            | ExpressionNode::LiteralNode { node: _ }
            | ExpressionNode::FunctionCall { node: _ } => match &token.token_type {
                tokenizer::TokenType::Operator { operator } => {
                    tokens.next();
                    *node = ExpressionNode::OperatorNode {
                        node: token_operator_to_node(
                            operator,
                            Some(Box::new((*node).clone())),
                            None,
                        ),
                    };
                }
                _ => panic!("{:?} {:?}", node, token),
            },

            ExpressionNode::OperatorNode { node } => {
                if node.right.is_none() {
                    node.right = Some(Box::from(tokens_to_expression_node_type(tokens)?));
                    stack.push(ExpressionNodeStep::OperatorRight);
                } else {
                    panic!("ASDASDASDA");
                }
            }

            ExpressionNode::StatementNode {
                node: StatementNode::Return { expression },
            } => {
                if expression.is_none() {
                    stack.push(ExpressionNodeStep::ReturnExpression);
                    *expression = Some(Box::new(tokens_to_expression_node_type(tokens)?));
                } else {
                    panic!("ASDASDASDA");
                }
            }

            ExpressionNode::StatementNode {
                node: StatementNode::If { condition, body },
            } => {
                println!("Expression created");
                return Ok(root);
            }

            _ => panic!("UNKNOWN NODE!!! {:?}", node),
        };
    }
    println!("Expression created");

    Ok(root)
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
            ExpressionNodeStep::ReturnExpression => match root {
                ExpressionNode::StatementNode {
                    node: StatementNode::Return { expression },
                } => node_at(expression.as_deref_mut().unwrap(), steps),
                _ => panic!("AAAAAAAAAA"),
            },
            _ => panic!("AAAAAAAAAA"),
        },
    }
}

fn create_block(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
) -> Result<Vec<ExpressionNode>, CompilerError> {
    let mut expressions: Vec<ExpressionNode> = Vec::new();

    while let next = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "Function not closed".to_string(),
        })?
        && next.token_type
            != (tokenizer::TokenType::Enclosing {
                enclosing_type: tokenizer::EnclosingType::CurlyBracket,
                open: false,
            })
    {
        expressions.push(create_expression_until_stop(tokens)?);
        tokens.next();
    }

    Ok(expressions)
}

fn token_literal_to_node(literal: &tokenizer::Literal) -> LiteralNode {
    match literal {
        tokenizer::Literal::IntLiteral { value } => LiteralNode::IntLiteral { number: *value },
    }
}

fn tokens_to_expression_node_type(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
) -> Result<ExpressionNode, CompilerError> {
    let token = tokens.next().unwrap();
    println!("asd {:?}", token);

    Ok(match &token.token_type {
        tokenizer::TokenType::Literal { literal } => match literal {
            tokenizer::Literal::IntLiteral { value } => ExpressionNode::LiteralNode {
                node: LiteralNode::IntLiteral { number: *value },
            },
        },
        tokenizer::TokenType::Identifier { identifier } => {
            if tokens.peek().is_some_and(|t| {
                t.token_type
                    == (tokenizer::TokenType::Enclosing {
                        enclosing_type: tokenizer::EnclosingType::Bracket,
                        open: true,
                    })
            }) {
                create_function_call(identifier.clone(), tokens)?
            } else {
                ExpressionNode::VariableNode {
                    node: VariableNode {
                        name: (*identifier).clone(),
                    },
                }
            }
        }
        tokenizer::TokenType::FlowControll {
            flow_controll_type: FlowControllType::Return,
        } => ExpressionNode::StatementNode {
            node: StatementNode::Return { expression: None },
        },
        tokenizer::TokenType::FlowControll {
            flow_controll_type: FlowControllType::If,
        } => {
            tokens.next();
            let cond = create_expression_until_stop(tokens)?;
            tokens.next();
            let block = create_block(tokens)?;
            ExpressionNode::StatementNode {
                node: StatementNode::If {
                    condition: Box::from(cond),
                    body: block,
                },
            }
        }
        _ => panic!("NO TIMPLEMENTRADAS!!!{:?}", &token),
    })
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
        tokenizer::Operator::EqualEqual => OperatorNode {
            operator: Operator::Equals,
            left,
            right,
        },
        _ => panic!("AAAA {:?}", operator),
    }
}

pub fn visualize_program(program: &Program) {
    println!("functions:");
    for def in &program.functions {
        print_indentaiton(1);
        println!("{}", def.identifier);
        for ex in &def.body {
            visualize_expression_node(ex, 2);
        }
    }
}

pub fn visualize_expression_node(node: &ExpressionNode, indentation: usize) {
    match node {
        ExpressionNode::FunctionCall { node } => {
            print_indentaiton(indentation);
            println!("{}", node.identifier);
            for arg in &node.arguments {
                visualize_expression_node(arg, indentation + 1);
            }
        }
        ExpressionNode::LiteralNode { node } => {
            print_indentaiton(indentation);
            println!(
                "{}",
                match node {
                    LiteralNode::IntLiteral { number } => number,
                }
            );
        }
        ExpressionNode::OperatorNode { node } => {
            print_indentaiton(indentation);
            println!(
                "{}",
                match node.operator {
                    Operator::Divide => "/",
                    Operator::Minus => "-",
                    Operator::Multiply => "*",
                    Operator::Plus => "+",
                    Operator::Equals => "==",
                }
            );
            if let Some(left) = &node.left {
                visualize_expression_node(left.as_ref(), indentation + 1);
            }
            if let Some(right) = &node.right {
                visualize_expression_node(right.as_ref(), indentation + 1);
            }
        }
        ExpressionNode::StatementNode { node } => match node {
            StatementNode::Return { expression } => {
                print_indentaiton(indentation);
                println!("Return");
                if let Some(e) = expression {
                    visualize_expression_node(e, indentation + 1);
                }
            }
            StatementNode::If { condition, body } => {
                print_indentaiton(indentation);
                println!("If");
                visualize_expression_node(condition, indentation + 1);
                for arg in body {
                    visualize_expression_node(arg, indentation + 1);
                }
            }
            _ => panic!("ASDASD"),
        },
        ExpressionNode::VariableNode { node } => {
            print_indentaiton(indentation);
            println!("{}", node.name);
        }
    }
}

fn print_indentaiton(indentation: usize) {
    let v = " ".repeat(indentation * 2);
    print!("{}", v);
}
