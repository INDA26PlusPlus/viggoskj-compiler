use core::num;
use std::collections::HashMap;

use crate::{
    error::CompilerError,
    parser::{self, ExpressionNode},
    transpiler::Instruction::BinaryOp,
};

type ValueId = usize;

#[derive(Debug)]
pub enum Instruction {
    BinaryOp {
        dest: ValueId,
        operator: parser::Operator,
    },
    SetIntegerLiteral {
        dest: ValueId,
        value: i32,
    },
    PushToStack {
        value: i32,
    },
    LoadArgument {
        id: ValueId,
    },
    LoadVariable {
        id: ValueId,
    },
    Return {
        id: ValueId,
    },
}

#[derive(Debug)]
struct LiState {
    next_id: usize,
    arg_count: usize,
    variables: HashMap<String, usize>,
}

pub fn func_to_li(func: &parser::FunctionDeclarationNode) -> Result<String, CompilerError> {
    // define function
    let mut out = ".method public static ".to_string();

    // add return type
    out += type_to_string(&func.return_type).as_str();
    out += " ";
    // add funcfion identifier
    out += func.identifier.as_str();

    // add args
    out += "(";

    out += func
        .args
        .iter()
        .map(|arg| type_to_string(&arg.arg_type) + " " + arg.identifier.as_str())
        .collect::<Vec<String>>()
        .join(", ")
        .as_str();

    out += ")";

    // create state
    let mut state = LiState {
        next_id: 0,
        arg_count: func.args.len(),
        variables: HashMap::new(),
    };

    let mut instructions = Vec::new();

    // arguments
    for arg in &func.args {
        state
            .variables
            .insert(arg.identifier.clone(), state.next_id);
        state.next_id += 1;
    }

    // all isntructions
    expression_to_il(func, &func.body[0], &mut state, &mut instructions)?;

    out += "{\n";

    // temp max stack
    out += ".maxstack 20\n\n";

    for i in instructions {
        println!("{:?}", i);
        out += instruction_to_text(&i).as_str();
        out += "\n";
    }

    out += "}\n";

    Ok(out)
}

fn expression_to_il(
    func: &parser::FunctionDeclarationNode,
    current_node: &ExpressionNode,
    state: &mut LiState,
    instructions: &mut Vec<Instruction>,
) -> Result<(), CompilerError> {
    match current_node {
        parser::ExpressionNode::FunctionCall { node } => {
            panic!("NOT IMPL");
        }
        parser::ExpressionNode::LiteralNode { node } => match node {
            parser::LiteralNode::IntLiteral { number } => {
                instructions.push(Instruction::PushToStack { value: *number });
            }
        },
        parser::ExpressionNode::OperatorNode { node } => {
            expression_to_il(
                func,
                node.left.clone().unwrap().as_ref(),
                state,
                instructions,
            )?;

            expression_to_il(
                func,
                node.right.clone().unwrap().as_ref(),
                state,
                instructions,
            )?;



            instructions.push(Instruction::BinaryOp {
                dest: state.next_id,
                operator: node.operator.clone(),
            });

            state.next_id += 1;
        }
        parser::ExpressionNode::StatementNode { node } => match node {
            parser::StatementNode::Return { expression } => {
                expression_to_il(
                    func,
                    expression.clone().unwrap().as_ref(),
                    state,
                    instructions,
                )?;
                instructions.push(Instruction::Return {
                    id: get_id(instructions.last().unwrap()).ok_or(CompilerError::InvalidReturnExpression)?,
                });
            }
            parser::StatementNode::If { condition, body } => {
                panic!("NOT IMPL");
            }
            _ => panic!("ASDASD"),
        },
        parser::ExpressionNode::VariableNode { node } => {
            let id = *state
                .variables
                .get(&node.identifier)
                .ok_or(CompilerError::BuildingError)?;
            if id < state.arg_count {
                instructions.push(Instruction::LoadArgument { id });
            } else {
                instructions.push(Instruction::LoadVariable { id });
            }
        }
    };

    Ok(())
}

fn type_to_string(t: &parser::Type) -> String {
    match t {
        parser::Type::Int => "int32",
        parser::Type::Void => "void",
    }
    .to_string()
}

fn get_id(inst: &Instruction) -> Option<usize> {
    match inst {
        Instruction::LoadVariable { id } => Some(*id),
        Instruction::LoadArgument { id } => Some(*id),
        Instruction::PushToStack { value } => None,
        Instruction::BinaryOp { dest, operator } => Some(*dest),
        Instruction::SetIntegerLiteral { dest, value } => Some(*dest),
        _ => panic!("not impl {:?}", inst),
    }
}

fn instruction_to_text(inst: &Instruction) -> String {
    match inst {
        BinaryOp { dest, operator } => operator_text(operator),
        Instruction::LoadArgument { id } => format!("ldarg.{}", id),
        Instruction::Return { id } => "ret".to_string(),
        Instruction::PushToStack { value } => format!("ldc.i4.{}", value),
        _ => panic!("Asdasdasd"),
    }
}

fn operator_text(op: &parser::Operator) -> String {
    match op {
        parser::Operator::Plus => "add".to_string(),
        _ => panic!("AASD"),
    }
}
