#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Segment {
    Separator,
    Segment(String),
    Dot,
    DotDot,
}
