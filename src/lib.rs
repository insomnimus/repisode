pub mod app;
pub mod cmd;
pub mod engine;

use std::io::{
    self,
    BufRead,
    Write,
};
#[cfg(windows)]
use std::{
    iter,
    path::PathBuf,
    process,
};

#[cfg(windows)]
fn is_glob(s: &str) -> bool {
    s.chars().any(|c| c == '*' || c == '?')
}

#[cfg(windows)]
pub fn get_files(m: &clap::ArgMatches) -> Vec<PathBuf> {
    use glob::MatchOptions;
    use itertools::Either;

    m.values_of("file")
        .unwrap()
        .map(|s| {
            if is_glob(s) {
                const OPTS: MatchOptions = MatchOptions {
                    case_sensitive: false,
                    require_literal_separator: true,
                    require_literal_leading_dot: true,
                };

                Either::Left(
                    glob::glob_with(s, OPTS)
                        .unwrap_or_else(|_| {
                            eprintln!("error: invalid pattern {}", s);
                            process::exit(2);
                        })
                        .map(|r| {
                            r.unwrap_or_else(|e| {
                                eprintln!("error: {}", e);
                                process::exit(2);
                            })
                        }),
                )
            } else {
                Either::Right(iter::once(PathBuf::from(s)))
            }
        })
        .flatten()
        .collect()
}

pub fn confirm(s: &str) -> bool {
    print!("{} [y/n]: ", s);
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let stdin = stdin.lock();

    for input in stdin.lines() {
        match &input.unwrap().to_lowercase()[..] {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("please enter y/n"),
        };
        print!("{} [y/n]: ", s);
        io::stdout().flush().unwrap();
    }
    false
}
