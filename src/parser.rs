use regex::Regex;
use std::ffi::OsStr;
use subparse::SubtitleEntry;

use crate::error::AppError;

#[derive(Debug, PartialEq)]
pub struct Match {
    pub line: String,
    pub start: usize,
    pub end: usize,
}

pub struct Parser<'a> {
    regex: &'a Regex,
    enteries: Vec<SubtitleEntry>,
}

impl<'a> Parser<'a> {
    pub fn new(regex: &Regex) -> Parser {
        Parser {
            regex,
            enteries: Vec::new(),
        }
    }

    pub fn set_content(
        &mut self,
        extension: Option<&OsStr>,
        content: String,
    ) -> Result<(), AppError> {
        if let Some(format) = subparse::get_subtitle_format(extension, content.as_bytes()) {
            self.enteries = subparse::parse_str(format, &content, 30.0)?.get_subtitle_entries()?;
        } else {
            return Err(AppError::new(
                "format".to_string(),
                "invalid file format".to_string(),
            ));
        }

        Ok(())
    }

    pub fn next(&mut self) -> Option<Match> {
        while !self.enteries.is_empty() {
            let entry = self.enteries.remove(0);

            if let Some(line) = entry.line {
                if let Some(mat) = self.regex.find(&line) {
                    return Some(Match {
                        line: line.clone(),
                        start: mat.start(),
                        end: mat.end(),
                    });
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn no_content() -> Result<(), Box<dyn Error>> {
        let regex = Regex::new(r"test")?;
        let mut parser = Parser::new(&regex);

        assert_eq!(None, parser.next());
        Ok(())
    }

    #[test]
    fn invalid_format() -> Result<(), Box<dyn Error>> {
        let regex = Regex::new(r"")?;
        let content = String::new();
        let mut parser = Parser::new(&regex);

        let result = parser.set_content(Some(OsStr::new("")), content);

        assert_eq!(
            result.err(),
            Some(AppError::new(
                "format".to_string(),
                "invalid file format".to_string()
            ))
        );
        Ok(())
    }

    #[test]
    fn next() -> Result<(), Box<dyn Error>> {
        let regex = Regex::new(r"final")?;
        let content = "\
1
00:02:17,440 --> 00:02:20,375
Senator, we're making
our final approach into Coruscant.
"
        .to_string();

        let mut parser = Parser::new(&regex);
        parser.set_content(Some(OsStr::new("srt")), content)?;

        let expected = Match {
            line: "\
Senator, we're making
our final approach into Coruscant."
                .to_string(),
            start: 26,
            end: 31,
        };

        assert_eq!(Some(expected), parser.next());
        Ok(())
    }
}
