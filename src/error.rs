use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Parse Error at \"{0}\"")]
    ParseError(String),

    #[error("Could not convert from an &std::path::Path")]
    ConvertError,

    #[error("Could not use the current environment to get the required path")]
    EnvError(#[from] std::io::Error),
}
