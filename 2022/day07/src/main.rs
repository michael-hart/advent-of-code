use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIR_RE: Regex = Regex::new("- (.*) \\(dir\\)").expect("Could not compile regex!");
    static ref FILE_RE: Regex = Regex::new("- (.*) \\(file, size=(\\d+)\\)").expect("Could not compile regex!");
}

struct Directory {
    name: String,
    dirs: Option<Vec<Directory>>,
    files: Option<Vec<File>>,
    // contents: Option<Vec<Box<dyn FileSystem>>>,
}

impl TryFrom<&str> for Directory {
    type Error = FileError;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let caps = FILE_RE.captures(text).ok_or(FileError)?;
        let name: String = caps.get(1).map_or("".into(), |m| m.as_str().into());
        Ok(Directory { name, dirs: None, files: None })
    }
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct FileError;

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to parse to file")
    }
}


impl TryFrom<&str> for File {
    type Error = FileError;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let caps = FILE_RE.captures(text).ok_or(FileError)?;
        let name: String = caps.get(1).map_or("".into(), |m| m.as_str().into());
        let size = caps.get(2).map_or("", |m| m.as_str()).parse::<usize>().map_err(|_| FileError)?;
        Ok(File { name, size })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_re() {
        let expected = Ok(File {name: "a".into(), size: 120});
        let actual = File::try_from("  - a (file, size=120)");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_directory_re() {
        let expected = Ok(Directory {name: "a".into(), dirs: None, files: None});
        let actual = Directory::try_from("    - a (dir)");
        assert_eq!(expected, actual);
    }
}
