use crate::tokenizer::Token;

#[derive(Debug)]
pub enum CompilerError {
    BuildingError,
    ExpressionError { reasong: String },
    InvalidFunctionDeclaration { reason: String },
    TokenizationError { position: usize },
    NotAnIdentifier { token: Token },
    InvalidReturnExpression,
    NotAType,
    FunctionNotDefined,
    NoBody,
}
