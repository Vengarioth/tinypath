use logos::Logos;
use super::{Segment, PathError};

#[derive(Debug)]
pub enum FileNameSegment {
    Separator,
    Segment(String),
}

#[derive(Debug, Logos)]
pub enum FileNameToken {
    #[regex(r"\.")]
    Separator,

    #[regex(r"[^\.]+")]
    Segment,

    #[error]
    Error,
}

impl FileNameToken {
    pub fn from_str(path: &str) -> Result<Vec<FileNameSegment>, PathError> {
        let mut segments = Vec::new();
        let mut lexer = FileNameToken::lexer(path);

        while let Some(token) = lexer.next() {
            match token {
                FileNameToken::Separator => {
                    segments.push(FileNameSegment::Separator);
                },
                FileNameToken::Segment => {
                    segments.push(FileNameSegment::Segment(lexer.slice().to_string()));
                },
                FileNameToken::Error => {
                    return Err(PathError::ParseError(lexer.slice().to_string()));
                },
            }
        }

        Ok(segments)
    }
}
