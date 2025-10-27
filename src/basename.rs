use clap::Parser;
use std::{env::ArgsOs, iter::Peekable};

#[derive(Parser)]
#[command(name = "basename")]
#[command(version = "1.0.0")]
#[command(about = "return non-directory portion of pathname")]
#[clap(disable_help_flag = true)]
#[clap(disable_version_flag = true)]
struct Basename {
    /// A string.
    string: String,
    /// A string.
    suffix: Option<String>,

    /// Print help.
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    /// Print version.
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// basename - return non-directory portion of pathname.
pub fn basename(args: Peekable<ArgsOs>) -> i32 {
    let Basename { string, suffix, .. } = match Basename::try_parse_from(args) {
        Ok(basename) => basename,
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

    let mut string = string.trim_end_matches('/');
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

    println!("{string}");
    0
}
