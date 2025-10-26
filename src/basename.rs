use clap::Parser;
use std::{env::ArgsOs, iter::Peekable};

#[derive(Parser)]
#[command(name = "basename")]
#[command(version = "1.0.0")]
#[command(about = "return non-directory portion of pathname")]
struct Basename {
    /// String containing a path.
    string: String,
    /// Optional suffix to strip from the base name.
    suffix: Option<String>,
}

/// basename - return non-directory portion of pathname.
pub fn basename(args: Peekable<ArgsOs>) -> i32 {
    let (string, suffix) = match Basename::try_parse_from(args) {
        Ok(Basename { string, suffix }) => (string, suffix),
        Err(err) => {
            eprint!("{err}");
            return 1;
        }
    };

    // Return '.' if the supplied string is empty.
    if string.is_empty() {
        println!(".");
        return 0;
    }

    // If the entire string is only slashes, return only a single slash.
    if string.find(|ch| ch != '/').is_none() {
        println!("/");
        return 0;
    }

    // Remove trailing slashes.
    let mut string = string.trim_end_matches('/');

    // Remove prefix.
    if let Some(idx) = string.rfind('/') {
        debug_assert!(idx < string.len());
        string = &string[idx + 1..];
    }

    // Optionally remove suffix if the suffix exists and is not equal to the string.
    if let Some(suffix) = suffix
        && string != suffix
        && string.ends_with(&suffix)
    {
        string = &string[..string.len() - suffix.len()];
    }

    // Write the result with newline to stdout.
    println!("{string}");
    0
}
