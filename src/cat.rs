use clap::Parser;
use std::{
    env::ArgsOs,
    fs::File,
    io::{BufReader, BufWriter, Error, Read, Write},
    iter::Peekable,
};

#[derive(Parser)]
#[command(name = "cat")]
#[command(version = "1.0.3")]
#[command(about = "concatenate and print files")]
#[clap(disable_help_flag = true)]
#[clap(disable_version_flag = true)]
struct Cat {
    /// Write bytes from the input file to the standard output without delay as each is
    /// read.
    #[arg(short)]
    unbuffered: bool,
    /// A pathname of an input file. If no file operands are specified, the standard input
    /// shall be used. If a file is '−', the cat utility shall read from the standard input at
    /// that point in the sequence. The cat utility shall not close and reopen standard input
    /// when it is referenced in this way, but shall accept multiple occurrences of '−' as a
    /// file operand.
    files: Vec<String>,

    /// Print help.
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    /// Print version.
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// cat - concatenate and print files.
pub fn cat(args: Peekable<ArgsOs>) -> i32 {
    let Cat {
        unbuffered, files, ..
    } = match Cat::try_parse_from(args) {
        Ok(cat) => cat,
        Err(err) => {
            eprint!("{err}");
            return 1;
        }
    };

    if unbuffered {
        if let Err(err) = unbuffered_cat(files) {
            eprintln!("{err}");
            return 1;
        }
    } else if let Err(err) = buffered_cat(files) {
        eprintln!("{err}");
        return 1;
    }

    0
}

fn unbuffered_cat(files: Vec<String>) -> Result<(), Error> {
    let mut stdout = std::io::stdout();

    // Use stdin if no files were supplied.
    if files.is_empty() {
        for byte in BufReader::new(std::io::stdin()).bytes() {
            stdout.write_all(&[byte?])?;
            stdout.flush()?;
        }

        return Ok(());
    }

    let mut stdin = None;
    for file in files {
        // Use stdin if the file is '-'.
        if file == "-" {
            // Only open stdin when needed, otherwise it is never opened.
            for byte in stdin.get_or_insert(std::io::stdin()).lock().bytes() {
                stdout.write_all(&[byte?])?;
                stdout.flush()?;
            }
        } else {
            for byte in BufReader::new(File::open(file)?).bytes() {
                stdout.write_all(&[byte?])?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

fn buffered_cat(files: Vec<String>) -> Result<(), Error> {
    let mut stdout = BufWriter::new(std::io::stdout().lock());

    // Use stdin if no files were supplied.
    if files.is_empty() {
        std::io::copy(&mut std::io::stdin(), &mut stdout)?;
        return Ok(());
    }

    let mut stdin = None;
    for file in files {
        // Use stdin if the file is '-'.
        if file == "-" {
            // Only open stdin when needed, otherwise it is never opened.
            std::io::copy(
                &mut stdin.get_or_insert(std::io::stdin()).lock(),
                &mut stdout,
            )?;
        } else {
            std::io::copy(&mut BufReader::new(File::open(file)?), &mut stdout)?;
        }
    }

    Ok(())
}
