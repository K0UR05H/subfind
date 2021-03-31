use std::{env, fs, path::PathBuf};

use ansi_term::Color::{Blue, Green, Red};
use clap::{App, Arg};
use regex::Regex;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

mod options {
    pub const DIR: &str = "directory";
    pub const PATTERN: &str = "pattern";
    pub const RECURSIVE: &str = "recursive";
}

mod error;

type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = match current_dir.to_str() {
        Some(path) => path,
        None => panic!("could not get the current working directory."),
    };

    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::with_name(options::PATTERN)
                .value_name("PATTERN")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name(options::DIR)
                .short("d")
                .long("directory")
                .value_name("DIR")
                .help("set a directory to search for the subtitles")
                .default_value(current_dir)
                .hide_default_value(true),
        )
        .arg(
            Arg::with_name(options::RECURSIVE)
                .short("r")
                .long("recursive")
                .help("search recursively"),
        )
        .get_matches();

    let pattern = matches.value_of(options::PATTERN).unwrap();
    let path = matches.value_of(options::DIR);
    let recursive = matches.is_present(options::RECURSIVE);

    let re = Regex::new(pattern)?;
    subs(&re, path, recursive)
}

fn subs(re: &Regex, path: Option<&str>, recursive: bool) -> Result<()> {
    if let Some(path) = path {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() && recursive {
                    subs(re, entry.path().to_str(), true)?
                } else if file_type.is_file() {
                    println!("{}: {}", Blue.paint("Opening"), entry.path().display());
                    if let Err(error) = find(re, entry.path()) {
                        eprintln!("{}: {}", Red.paint("Error"), error);
                    }
                }
            }
        }
    }

    Ok(())
}

fn find(re: &Regex, path: PathBuf) -> Result<()> {
    let file_content = fs::read_to_string(&path)?;

    if let Some(format) = subparse::get_subtitle_format(path.extension(), file_content.as_bytes()) {
        let subtitle_file = subparse::parse_str(format, &file_content, 30.0)?;
        let subtitle_entries = subtitle_file.get_subtitle_entries()?;

        for subtitle_entry in subtitle_entries {
            if let Some(line) = subtitle_entry.line {
                if let Some(mat) = re.find(&line) {
                    println!(
                        "{}{}{}",
                        &line[..mat.start()],
                        Green.paint(&line[mat.start()..mat.end()]),
                        &line[mat.end()..]
                    );
                }
            }
        }
    }

    Ok(())
}
