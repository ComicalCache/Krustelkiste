use clap::Parser;
use std::{
    env::ArgsOs,
    fs::File,
    io::{BufReader, Error, Read},
    iter::Peekable,
};

macro_rules! cmp {
    ($list:ident, $short:ident, $file1:ident, $file2:ident, $path1:ident, $path2:ident) => {{
        let mut bytes1 = $file1.bytes();
        let mut bytes2 = $file2.bytes();
        let mut count1 = 0;
        let mut count2 = 0;

        let mut ret = 0;
        let mut line = 1;
        for idx in 1.. {
            // Traversing the bytes manually like this means that the counts will differ if the two files are not of
            // identical length.
            let byte1 = match bytes1.next() {
                Some(byte1) => {
                    count1 += 1;
                    Some(byte1?)
                }
                None => None,
            };
            let byte2 = match bytes2.next() {
                Some(byte2) => {
                    count2 += 1;
                    Some(byte2?)
                }
                None => None,
            };
            let Some(byte1) = byte1 else {
                break;
            };
            let Some(byte2) = byte2 else {
                break;
            };

            // Continue if the bytes are identical.
            if byte1 == byte2 {
                // Count lines on new lines.
                if byte1 == b'\n' {
                    line += 1;
                }

                continue;
            }

            // Short circuit and only return status.
            if $short {
                return Ok(1);
            }

            if $list {
                // Print all different bytes.
                println!("{idx} {byte1:o} {byte2:o}");
                ret = 1;
            } else {
                // Print the first difference and exit.
                println!("{} {} differ: char {idx}, line {line}", $path1, $path2);
                return Ok(1);
            }
        }

        // Print EOF message when files are not identical in length.
        if count1 != count2 && ($list || ret == 0) {
            #[allow(clippy::redundant_else)]
            if count1 < count2 {
                eprintln!("cmp: EOF on {}", $path1);
                return Ok(1);
            } else {
                eprintln!("cmp: EOF on {}", $path2);
                return Ok(1);
            }
        }

        Ok(ret)
    }};
}

#[derive(Parser)]
#[command(name = "cmp")]
#[command(version = "1.0.0")]
#[command(about = "compare two files")]
#[clap(disable_help_flag = true)]
#[clap(disable_version_flag = true)]
struct Cmp {
    /// (Lowercase ell.) Write the byte number (decimal) and the differing bytes (octal) for each difference.
    #[arg(short)]
    list: bool,
    /// Write nothing for differing files; return exit status only.
    #[arg(short)]
    short: bool,
    /// A pathname of the first file to be compared. if file1 is '-', the standard input shall be used.
    file1: String,
    /// A pathname of the second file to be compared. if file2 is '-', the standard input shall be used.
    file2: String,

    /// Print help.
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    /// Print version.
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// cat - concatenate and print files.
pub fn cmp(args: Peekable<ArgsOs>) -> i32 {
    let Cmp {
        list,
        short,
        file1,
        file2,
        ..
    } = match Cmp::try_parse_from(args) {
        Ok(cat) => cat,
        Err(err) => {
            eprint!("{err}");
            return 1;
        }
    };

    // Return identical if both files are stdin.
    if file1 == "-" && file2 == "-" {
        return 0;
    }

    match __cmp(list, short, file1.as_str(), file2.as_str()) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            2
        }
    }
}

fn __cmp(list: bool, short: bool, path1: &str, path2: &str) -> Result<i32, Error> {
    if path1 == "-" {
        // Use stdin for the first file.
        let file1 = BufReader::new(std::io::stdin());
        let file2 = BufReader::new(File::open(path2)?);
        let path1 = "stdin";

        cmp!(list, short, file1, file2, path1, path2)
    } else if path2 == "-" {
        // Use stdin for the second file.
        let file1 = BufReader::new(File::open(path1)?);
        let file2 = BufReader::new(std::io::stdin());
        let path2 = "stdin";

        cmp!(list, short, file1, file2, path1, path2)
    } else {
        let file1 = BufReader::new(File::open(path1)?);
        let file2 = BufReader::new(File::open(path2)?);

        cmp!(list, short, file1, file2, path1, path2)
    }
}
