use crate::Segment;
use std::path::PathBuf;

pub struct Path {
    segments: Vec<Segment>,
}

impl Path {
    pub fn new(segments: Vec<Segment>) -> Self {
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

    pub fn append(&self, append: Self) -> Self {
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
