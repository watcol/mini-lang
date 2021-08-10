use peg::{error::ParseError, str::LineCol};

pub type MiniResult<T> = Result<T, MiniError>;

#[derive(thiserror::Error, Debug)]
pub enum MiniError {
    #[error("Parse Error: {0}")]
    Parse(ParseError<LineCol>),
    #[error("Execution Error: {0}")]
    Execution(String),
    #[error("{0}")]
    Any(Box<dyn std::error::Error>)
}

impl MiniError {
    pub fn from_error<E: std::error::Error + 'static>(error: E) -> Self {
        Self::Any(Box::new(error))
    }
}

impl<T: Into<String>> From<T> for MiniError {
    fn from(s: T) -> Self {
        Self::Execution(s.into())
    }
}
