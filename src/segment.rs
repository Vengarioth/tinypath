#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Segment {
    Separator,
    Segment(String),
    Dot,
    DotDot,
}

impl Segment {
    pub fn get_segment(&self) -> Option<&str> {
        match self {
            Segment::Segment(ref segment) => Some(segment),
            _ => None,
        }
    }

    pub fn is_separator(&self) -> bool {
        match self {
            Segment::Separator => true,
            _ => false,
        }
    }
}
