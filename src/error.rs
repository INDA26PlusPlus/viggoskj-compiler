#[derive(Debug)]
pub enum CompilerError {
    BuildingError,
    TokenizationError { position: usize },
}