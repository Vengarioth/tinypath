mod error;
mod segment;
mod path;
mod token;

use segment::Segment;
use token::Token;
pub use error::PathError;
pub use path::Path;

pub fn parse(path: &str) -> Result<Path, PathError> {
    let segments = Token::from_str(path)?;
    Ok(Path::new(segments))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_platform_string() {
        let path = parse("C:/foo/bar/../").unwrap();
        assert_eq!(path.dedot().to_platform_string(), "C:\\foo\\");
    }

    #[test]
    fn it_dedots() {
        let path = parse("C:/foo/bar/../").unwrap();
        assert_eq!(path.dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_dedots_backslash() {
        let path = parse("C:\\foo\\bar\\..\\").unwrap();
        assert_eq!(path.dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends() {
        let path = parse("C:/").unwrap();
        let append = parse("./foo/").unwrap();
        assert_eq!(path.append(append).dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends_backslash() {
        let path = parse("C:\\").unwrap();
        let append = parse(".\\foo\\").unwrap();
        assert_eq!(path.append(append).dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends_mixed() {
        let path = parse("C:\\").unwrap();
        let append = parse("./foo/").unwrap();
        assert_eq!(path.append(append).dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_pushes() {
        let mut path = parse("C:/foo").unwrap();
        path.push("bar");
        assert_eq!(path.to_string(), "C:/foo/bar");
    }

    #[test]
    fn it_pushes_with_leading_slash() {
        let mut path = parse("C:/foo/").unwrap();
        path.push("bar");
        assert_eq!(path.to_string(), "C:/foo/bar");
    }

    #[test]
    fn it_pops() {
        let mut path = parse("C:/foo/bar").unwrap();
        path.pop();
        assert_eq!(path.to_string(), "C:/foo/");
    }
    
    #[test]
    fn it_pops_with_leading_slash() {
        let mut path = parse("C:/foo/bar/").unwrap();
        path.pop();
        assert_eq!(path.to_string(), "C:/foo/");
    }
}
