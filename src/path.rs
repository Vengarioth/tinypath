use crate::{Segment, Token, PathError};
use std::path::{PathBuf, Path as StdPath};
use std::fmt;

pub struct Path {
    segments: Vec<Segment>,
}

impl Path {
    pub fn from_str(path: &str) -> Result<Self, PathError> {
        let segments = Token::from_str(path)?;
        Ok(Self::new(segments))
    }

    pub fn from_std_path(path: &StdPath) -> Result<Self, PathError> {
        let path = path.as_os_str().to_str().ok_or(PathError::ConvertError)?;
        let segments = Token::from_str(path)?;
        Ok(Self::new(segments))
    }

    pub(crate) fn new(segments: Vec<Segment>) -> Self {
        Self {
            segments,
        }
    }

    pub fn to_platform_string(&self) -> String {
        let mut result = String::new();

        for segment in self.segments.iter() {
            match segment {
                Segment::Separator => {
                    result.push(std::path::MAIN_SEPARATOR);
                },
                Segment::Segment(value) => {
                    result.push_str(value);
                },
                Segment::Dot => {
                    result.push_str(".");
                },
                Segment::DotDot => {
                    result.push_str("..");
                }
            }
        }

        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for segment in self.segments.iter() {
            match segment {
                Segment::Separator => {
                    result.push_str("/");
                },
                Segment::Segment(value) => {
                    result.push_str(value);
                },
                Segment::Dot => {
                    result.push_str(".");
                },
                Segment::DotDot => {
                    result.push_str("..");
                }
            }
        }

        result
    }

    pub fn dedot(&self) -> Self {
        let mut segments = Vec::new();
        
        let mut skip_next_separator = false;
        for segment in self.segments.iter() {
            match segment {
                Segment::Separator => {
                    if skip_next_separator {
                        skip_next_separator = false;
                    } else {
                        segments.push(Segment::Separator);
                    }
                },
                Segment::Segment(value) => {
                    segments.push(Segment::Segment(value.to_string()));
                },
                Segment::Dot => {
                    skip_next_separator = true;
                },
                Segment::DotDot => {
                    segments.pop();
                    segments.pop();
                    skip_next_separator = true;
                }
            }
        }

        Self::new(segments)
    }

    pub fn relative_from(&self, base: &Self) -> Self {
        let mut base_segments = base.segments.clone();

        if base_segments.len() > 0 && base_segments[base_segments.len() - 1] != Segment::Separator {
            base_segments.pop();
        }

        let mut segments = self.segments.clone();

        while base_segments.len() > 0 && segments.len() > 0 && base_segments[0] == segments[0] {
            base_segments.remove(0);
            segments.remove(0);
        }

        segments.insert(0, Segment::Separator);
        segments.insert(0, Segment::Dot);

        Self::new(segments)
    }

    pub fn relative_to(&self, base: &Self) -> Self {
        let mut segments = base.segments.clone();

        if segments.len() > 0 && segments[segments.len() - 1] != Segment::Separator {
            segments.pop();
        }

        let mut skip_next_separator = true;
        for segment in self.segments.iter() {
            match segment {
                Segment::Separator => {
                    if skip_next_separator {
                        skip_next_separator = false;
                    } else {
                        segments.push(Segment::Separator);
                    }
                },
                Segment::Segment(value) => {
                    segments.push(Segment::Segment(value.to_string()));
                },
                Segment::Dot => {
                    skip_next_separator = true;
                },
                Segment::DotDot => {
                    segments.pop();
                    segments.pop();
                    skip_next_separator = true;
                }
            }
        }

        Self::new(segments)
    }

    pub fn append(&self, append: &Self) -> Self {
        let mut segments = self.segments.clone();
        segments.append(&mut append.segments.clone());
        Self::new(segments)
    }

    pub fn push(&mut self, value: impl Into<String>) {
        if self.segments.len() > 0 && self.segments[self.segments.len() - 1] != Segment::Separator {
            self.segments.push(Segment::Separator);
        }

        self.segments.push(Segment::Segment(value.into()));
    }

    pub fn pop(&mut self) {
        if self.segments.len() > 0 && self.segments[self.segments.len() - 1] == Segment::Separator {
            self.segments.pop();
        }

        if self.segments.len() > 0 {
            self.segments.pop();
        }
    }
}

impl Into<PathBuf> for Path {
    fn into(self) -> PathBuf {
        PathBuf::from(self.to_platform_string())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
