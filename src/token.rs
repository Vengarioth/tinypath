use logos::Logos;
use super::{Segment, PathError};

#[derive(Debug, Logos)]
pub enum Token {
    #[regex(r"[\\/]+")]
    Separator,

    #[regex(r"[^\\/]+")]
    Segment,

    #[regex(r"\.")]
    Dot,

    #[regex(r"\.\.")]
    DotDot,

    #[error]
    Error,
}

impl Token {
    pub fn from_str(path: &str) -> Result<Vec<Segment>, PathError> {
        let mut segments = Vec::new();
        let mut lexer = Token::lexer(path);

        while let Some(token) = lexer.next() {
            match token {
                Token::Separator => {
                    segments.push(Segment::Separator);
                },
                Token::Segment => {
                    segments.push(Segment::Segment(lexer.slice().to_string()));
                },
                Token::Dot => {
                    segments.push(Segment::Dot);
                },
                Token::DotDot => {
                    segments.push(Segment::DotDot);
                },
                Token::Error => {
                    return Err(PathError::ParseError(lexer.slice().to_string()));
                },
            }
        }

        Ok(segments)
    }
}
