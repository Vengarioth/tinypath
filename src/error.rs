use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Parse Error at \"{0}\"")]
    ParseError(String),
}
