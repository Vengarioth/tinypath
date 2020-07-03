mod error;
mod segment;
mod path;
mod token;
mod file_name_token;

use segment::Segment;
use token::Token;
use file_name_token::{FileNameToken, FileNameSegment};
pub use error::PathError;
pub use path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_platform_string() {
        let path = Path::from_str("C:/foo/bar/../").unwrap();
        assert_eq!(path.dedot().to_platform_string(), "C:\\foo\\");
    }

    #[test]
    fn it_dedots() {
        let path = Path::from_str("C:/foo/bar/../").unwrap();
        assert_eq!(path.dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_dedots_backslash() {
        let path = Path::from_str("C:\\foo\\bar\\..\\").unwrap();
        assert_eq!(path.dedot().to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends() {
        let path = Path::from_str("C:/").unwrap();
        let append = Path::from_str("./foo/").unwrap();
        assert_eq!(path.append(&append).to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends_backslash() {
        let path = Path::from_str("C:\\").unwrap();
        let append = Path::from_str(".\\foo\\").unwrap();
        assert_eq!(path.append(&append).to_string(), "C:/foo/");
    }

    #[test]
    fn it_appends_mixed() {
        let path = Path::from_str("C:\\").unwrap();
        let append = Path::from_str("./foo/").unwrap();
        assert_eq!(path.append(&append).to_string(), "C:/foo/");
    }

    #[test]
    fn it_pushes() {
        let mut path = Path::from_str("C:/foo").unwrap();
        path.push("bar");
        assert_eq!(path.to_string(), "C:/foo/bar");
    }

    #[test]
    fn it_pushes_with_leading_slash() {
        let mut path = Path::from_str("C:/foo/").unwrap();
        path.push("bar");
        assert_eq!(path.to_string(), "C:/foo/bar");
    }

    #[test]
    fn it_pops() {
        let mut path = Path::from_str("C:/foo/bar").unwrap();
        path.pop();
        assert_eq!(path.to_string(), "C:/foo/");
    }
    
    #[test]
    fn it_pops_with_leading_slash() {
        let mut path = Path::from_str("C:/foo/bar/").unwrap();
        path.pop();
        assert_eq!(path.to_string(), "C:/foo/");
    }

    #[test]
    fn it_converts_from_path_buf() {
        let path = std::path::PathBuf::from("C:/foo/bar/");
        assert_eq!(Path::from_std_path(&path).unwrap().to_string(), "C:/foo/bar/");
    }

    #[test]
    fn it_converts_to_path_buf() {
        let path = Path::from_str("C:/foo/bar/").unwrap();
        let std_path: std::path::PathBuf = path.into();
        assert_eq!(std::path::PathBuf::from("C:/foo/bar/"), std_path);
    }

    #[test]
    fn it_becomes_relative_to_a_folder() {
        let path = Path::from_str("./foo").unwrap();
        let base = Path::from_str("C:/bar/").unwrap();

        assert_eq!(path.relative_to(&base).to_string(), "C:/bar/foo");
    }

    #[test]
    fn it_becomes_relative_to_a_file() {
        let path = Path::from_str("./foo.bar").unwrap();
        let base = Path::from_str("C:/bar/baz.foo").unwrap();

        assert_eq!(path.relative_to(&base).to_string(), "C:/bar/foo.bar");
    }

    #[test]
    fn it_becomes_relative_from_a_base_file() {
        let path = Path::from_str("C:/bar/foo.bar").unwrap();
        let base = Path::from_str("C:/bar/baz.bb").unwrap();

        assert_eq!(path.relative_from(&base).to_string(), "./foo.bar");
    }

    #[test]
    fn it_becomes_relative_from_a_base_folder() {
        let path = Path::from_str("C:/bar/foo.bar").unwrap();
        let base = Path::from_str("C:/bar/").unwrap();

        assert_eq!(path.relative_from(&base).to_string(), "./foo.bar");
    }

    #[test]
    fn it_implements_display() {
        let path = Path::from_str("C:/bar/foo.bar").unwrap();
        let as_string = format!("{}", path);

        assert_eq!(as_string, "C:/bar/foo.bar");
    }

    #[test]
    fn it_parses_extensions() {
        assert_eq!(Some("bar".to_string()), Path::from_str("./foo.bar").unwrap().extension());
        assert_eq!(Some("".to_string()), Path::from_str("./foo.").unwrap().extension());
        assert_eq!(None, Path::from_str("./.").unwrap().extension());
        assert_eq!(None, Path::from_str("./.foo").unwrap().extension());
        assert_eq!(Some("foo".to_string()), Path::from_str("./.bar.foo").unwrap().extension());
        assert_eq!(Some("foo".to_string()), Path::from_str("./.bar.foo.bar.foo.bar.foo").unwrap().extension());
        assert_eq!(None, Path::from_str("./").unwrap().extension());
        assert_eq!(None, Path::from_str("../").unwrap().extension());
    }
}
