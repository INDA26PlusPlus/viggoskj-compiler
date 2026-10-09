#[derive(Debug)]
pub enum CompilerError {
    BuildingError,
    ExpressionError { reasong: String },
    InvalidFunctionDeclaration { reason: String },
    TokenizationError { position: usize },
}