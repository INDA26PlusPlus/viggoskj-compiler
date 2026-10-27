use core::num;
use std::collections::HashMap;

use crate::{
    error::CompilerError,
    parser::{self, ExpressionNode, FunctionDeclarationNode, Program},
    transpiler::Instruction::BinaryOp,
};

type ValueId = usize;

#[derive(Debug)]
pub enum Instruction {
    BinaryOp {
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
    Return,
    FunctionCall {
        identifier: String,
        return_type: parser::Type,
        args: Vec<parser::Type>,
    },
}

#[derive(Debug)]
struct LiState<'a> {
    next_id: usize,
    arg_count: usize,
    variables: HashMap<String, usize>,
    functions: &'a Vec<FunctionDeclarationNode>,
}

pub fn prog_to_li(program: Program) -> Result<String, CompilerError> {
    let mut li = "
.assembly extern System.Runtime {}
.assembly extern System.Console {}

.assembly MyProgram {}

.class public auto ansi CompiledClass
    extends [System.Runtime]System.Object
{


    .method public static int32 Main() cil managed
    {
        .entrypoint
        .maxstack 1

        call int32 CompiledClass::main()

        call void [System.Console]System.Console::WriteLine(int32)

        ldc.i4.0
        ret
    }


    "
    .to_string();

    for func in &program.functions {
        li += "\n";
        li += func_to_li(func, &program.functions)?.as_str();
    }

    li += "\n}";

    return Ok(li);
}

pub fn func_to_li(
    func: &parser::FunctionDeclarationNode,
    funcs: &Vec<FunctionDeclarationNode>,
) -> Result<String, CompilerError> {
    // define function
    let mut out = indentation(1) + ".method public static ";

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
        functions: funcs,
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

    out += "\n";
    out += indentation(1).as_str();
    out += "{\n";

    // temp max stack
    out += indentation(2).as_str();
    out += ".maxstack 20\n\n";

    for i in instructions {
        println!("{:?}", i);
        out += indentation(2).as_str();
        out += instruction_to_text(&i).as_str();
        out += "\n";
    }

    out += indentation(1).as_str();
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
            let f = state
                .functions
                .iter()
                .find(|f| f.identifier == node.identifier)
                .ok_or(CompilerError::FunctionNotDefined)?;

            for arg in &node.arguments {
                expression_to_il(func, arg, state, instructions)?;
            }

            instructions.push(Instruction::FunctionCall {
                identifier: node.identifier.clone(),
                return_type: f.return_type.clone(),
                args: Vec::from_iter(f.args.iter().map(|a| a.arg_type.clone())),
            });
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
                operator: node.operator.clone(),
            });
        }
        parser::ExpressionNode::StatementNode { node } => match node {
            parser::StatementNode::Return { expression } => {
                expression_to_il(
                    func,
                    expression.clone().unwrap().as_ref(),
                    state,
                    instructions,
                )?;

                if let Some(variable) = get_id(instructions.last().unwrap()) {
                    instructions.push(Instruction::LoadVariable { id: variable });
                }

                instructions.push(Instruction::Return);
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
        Instruction::BinaryOp { operator } => None,
        Instruction::SetIntegerLiteral { dest, value } => Some(*dest),
        Instruction::FunctionCall { identifier, return_type, args } => None,
        _ => panic!("not impl {:?}", inst),
    }
}

fn instruction_to_text(inst: &Instruction) -> String {
    match inst {
        BinaryOp { operator } => operator_text(operator),
        Instruction::LoadArgument { id } => format!("ldarg.{}", id),
        Instruction::Return => "ret".to_string(),
        Instruction::PushToStack { value } => format!("ldc.i4 {}", value),
        Instruction::FunctionCall {
            identifier,
            args,
            return_type,
        } => format!(
            "call {} CompiledClass::{}({})",
            type_to_string(return_type),
            identifier,
            Vec::from_iter(args.iter().map(|a| type_to_string(a))).join(", ")
        ),
        _ => panic!("Asdasdasd {:?}", inst),
    }
}

fn operator_text(op: &parser::Operator) -> String {
    match op {
        parser::Operator::Plus => "add".to_string(),
        _ => panic!("AASD"),
    }
}

fn indentation(indentation: usize) -> String {
    "    ".repeat(indentation).to_string()
}
