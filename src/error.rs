#[derive(Debug)]
pub enum CompilerError {
    TokenizationError { position: usize },
}