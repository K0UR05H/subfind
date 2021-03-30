use std::{env, fs, io, path::PathBuf};

use clap::{App, Arg};

use ansi_term::Color::Blue;
use ansi_term::Color::Green;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

mod options {
    pub const DIR: &str = "directory";
    pub const PATTERN: &str = "pattern";
    pub const RECURSIVE: &str = "recursive";
}

fn main() -> io::Result<()> {
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

    subs(pattern, path, recursive)
}

fn subs(pattern: &str, path: Option<&str>, recursive: bool) -> io::Result<()> {
    if let Some(path) = path {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if recursive {
                        subs(pattern, entry.path().to_str(), true)?
                    }
                } else if file_type.is_file() {
                    println!("{}: {}", Blue.paint("Opening"), entry.path().display());
                    find(pattern, entry.path())?
                }
            }
        }
    }

    Ok(())
}

fn find(pattern: &str, path: PathBuf) -> io::Result<()> {
    let file_content = fs::read_to_string(&path)?;

    if let Some(format) = subparse::get_subtitle_format(path.extension(), file_content.as_bytes()) {
        let subtitle_file = subparse::parse_str(format, &file_content, 30.0).expect("parser error");
        let subtitle_entries = subtitle_file
            .get_subtitle_entries()
            .expect("unexpected error");

        for subtitle_entry in subtitle_entries {
            if let Some(line) = subtitle_entry.line {
                if let Some(index) = line.find(pattern) {
                    println!(
                        "{}{}{}",
                        &line[..index],
                        Green.paint(&line[index..index + pattern.len()]),
                        &line[index + pattern.len()..]
                    );
                }
            }
        }
    }

    Ok(())
}
