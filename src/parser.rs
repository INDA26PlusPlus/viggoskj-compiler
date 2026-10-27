use crate::error::*;
use crate::tokenizer;
use crate::tokenizer::EnclosingType;
use crate::tokenizer::FlowControllType;
use std::iter::Peekable;

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
        body: Option<Vec<ExpressionNode>>,
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
    pub identifier: String,
    pub arguments: Vec<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct OperatorNode {
    pub operator: Operator,
    pub left: Option<Box<ExpressionNode>>,
    pub right: Option<Box<ExpressionNode>>,
}

#[derive(Debug, Clone)]
pub struct VariableNode {
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Int,
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
    pub identifier: String,
    pub return_type: Type,
    pub args: Vec<ArgumentDeclaration>,
    pub body: Vec<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct ArgumentDeclaration {
    pub identifier: String,
    pub arg_type: Type,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDeclarationNode>,
}

pub fn parse_program(tokens: Vec<tokenizer::Token>) -> Result<Program, CompilerError> {
    let mut prog = Program {
        functions: Vec::new(),
    };

    let mut iter = tokens.iter().peekable();

    while iter.peek().is_some() {
        prog.functions.push(parse_function_declaration(&mut iter)?);
    }

    Ok(prog)
}

fn parse_function_declaration(
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

    let identifier = match &function_name_token.token_type {
        tokenizer::TokenType::Identifier { identifier } => identifier.clone(),
        _ => {
            return Err(CompilerError::InvalidFunctionDeclaration {
                reason: "invalid name token".to_string(),
            });
        }
    };

    println!("--------------------------------------- {}", identifier);

    let _ = tokens
        .next()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "".to_string(),
        })?;

    let mut args: Vec<ArgumentDeclaration> = Vec::new();

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
    {
        let identifier = tokens
            .next()
            .ok_or(CompilerError::InvalidFunctionDeclaration {
                reason: "function declaraiton end to soon".to_string(),
            })?;
        args.push(ArgumentDeclaration {
            identifier: as_identifier(identifier)?,
            arg_type: as_type(arg_type)?,
        });

        tokens.next_if(|t| {
            t.token_type
                == (tokenizer::TokenType::SyntaxFlow {
                    syntax_flow_type: tokenizer::SyntaxFlowType::Comma,
                })
        });
    }

    let expressions = parse_code_block(tokens, 0)?;

    tokens.next();

    Ok(FunctionDeclarationNode {
        identifier,
        return_type: as_type(return_type_token)?,
        args: args,
        body: expressions,
    })
}

fn parse_function_call(
    identifier: String,
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
    depth: usize,
) -> Result<ExpressionNode, CompilerError> {
    let bracket = tokens
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
        println!("--- {:?}", next.token_type);
        arguments.push(parse_expression_until_stop(tokens, depth)?);

        tokens.next_if(|f| f.token_type == (tokenizer::TokenType::SyntaxFlow { syntax_flow_type: tokenizer::SyntaxFlowType::Comma }));
    }

    tokens.next();

    Ok(ExpressionNode::FunctionCall {
        node: FunctionCallNode {
            arguments,
            identifier,
        },
    })
}

fn parse_expression_until_stop(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
    depth: usize
) -> Result<ExpressionNode, CompilerError> {
    let mut stack: Vec<ExpressionNodeStep> = Vec::new();
    println!("Creating expression ({}): {:?}", depth, tokens.peek());
    let mut root = tokens_to_expression_node_type(tokens, depth)?;
    println!("next: {:?}", tokens.peek());

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
                println!("Expression created: {:?}", token);
                visualize_expression_node(&root, 0);
                return Ok(root);
            }
            _ => {}
        }

        let node = node_at(&mut root, &mut stack.iter());
        println!("Building on {:?}", node);
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

            ExpressionNode::OperatorNode { node: n } => {
                if n.left.is_none() {
                    n.left = Some(Box::from(tokens_to_expression_node_type(tokens, depth)?));
                } else if n.right.is_none() {
                    n.right = Some(Box::from(tokens_to_expression_node_type(tokens, depth)?));
                } else {
                    match &token.token_type {
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
                    }
                }
            }

            ExpressionNode::StatementNode {
                node: StatementNode::Return { expression },
            } => {
                if expression.is_none() {
                    stack.push(ExpressionNodeStep::ReturnExpression);
                    let expr = tokens_to_expression_node_type(tokens, depth)?;

                    *expression = Some(Box::new(expr));
                } else {
                    panic!("ASDASDASDA");
                }
            }

            ExpressionNode::StatementNode {
                node: StatementNode::If { condition, body },
            } => {
                *body = Some(parse_code_block(tokens, depth)?);
                return Ok(root);
            }

            _ => panic!("UNKNOWN NODE!!! {:?}", node),
        };
        visualize_expression_node(&root, 5);
    }

    Err(CompilerError::ExpressionError {
        reasong: "Invalid expression ending".to_string(),
    })
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

fn parse_code_block(
    tokens: &mut Peekable<std::slice::Iter<tokenizer::Token>>,
    depth: usize,
) -> Result<Vec<ExpressionNode>, CompilerError> {
    let mut expressions: Vec<ExpressionNode> = Vec::new();
    println!("----------");
    tokens.next();
    while let next = tokens
        .peek()
        .ok_or(CompilerError::InvalidFunctionDeclaration {
            reason: "Function not closed".to_string(),
        })?
        && next.token_type
            != (tokenizer::TokenType::Enclosing {
                enclosing_type: tokenizer::EnclosingType::CurlyBracket,
                open: false,
            })
    {
        println!("###### Next expression {:?}", next);
        expressions.push(parse_expression_until_stop(tokens, depth)?);
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
    depth: usize
) -> Result<ExpressionNode, CompilerError> {
    let token = tokens.next().unwrap();
    println!("to expression node {:?}", token);

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
                parse_function_call(identifier.clone(), tokens, depth)?
            } else {
                ExpressionNode::VariableNode {
                    node: VariableNode {
                        identifier: (*identifier).clone(),
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
            let cond = parse_expression_until_stop(tokens, depth)?;
            tokens.next();

            ExpressionNode::StatementNode {
                node: StatementNode::If {
                    condition: Box::from(cond),
                    body: None,
                },
            }
        }
        tokenizer::TokenType::Operator { operator } => ExpressionNode::OperatorNode {
            node: token_operator_to_node(operator, None, None),
        },
        tokenizer::TokenType::Enclosing {
            enclosing_type: EnclosingType::Bracket,
            open: true,
        } => {
            let expr = parse_expression_until_stop(tokens, depth)?;
            tokens.next();
            expr
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
        print_indentaiton(2);
        println!("return {:?}", def.return_type);
        for arg in &def.args {
            print_indentaiton(2);
            println!("{:?} {:?}", arg.arg_type, arg.identifier)
        }
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
                if let Some(b) = body {
                    for arg in b {
                        visualize_expression_node(arg, indentation + 1);
                    }
                }
            }
            _ => panic!("ASDASD"),
        },
        ExpressionNode::VariableNode { node } => {
            print_indentaiton(indentation);
            println!("{}", node.identifier);
        }
    }
}

fn print_indentaiton(indentation: usize) {
    let v = "| ".repeat(indentation);
    print!("{}", v);
}

fn as_identifier(token: &tokenizer::Token) -> Result<String, CompilerError> {
    match &token.token_type {
        tokenizer::TokenType::Identifier { identifier } => Ok(identifier.clone()),
        _ => Err(CompilerError::NotAnIdentifier),
    }
}

fn as_type(token: &tokenizer::Token) -> Result<Type, CompilerError> {
    to_type(as_identifier(token)?)
}

fn to_type(identifier: String) -> Result<Type, CompilerError> {
    match identifier.as_str() {
        "int" => Ok(Type::Int),
        _ => Err(CompilerError::NotAType),
    }
}
