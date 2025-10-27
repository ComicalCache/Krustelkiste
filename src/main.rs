mod basename;
mod cat;

use crate::{basename::basename, cat::cat};
use std::process::exit;

macro_rules! setup {
    ($($cmd:ident),+) => {{
        let mut args = std::env::args_os().peekable();

        // Peek the item since clap assumes the first argument to be the callee.
        let Some(callee) = args.peek() else {
            eprintln!("No utility was specified");
            exit(1);
        };

        // Extract the command from the callee string.
        let mut cmd = {
            let cmd = callee.to_string_lossy();
            let mut cmd = cmd.trim_end_matches('/');
            if cmd.is_empty() {
                eprintln!("No utility was specified");
                exit(1);
            }

            // Trim the prefix.
            if let Some(idx) = cmd.rfind('/') {
                debug_assert!(idx < cmd.len());
                cmd = &cmd[idx + 1..];
            }

            String::from(cmd)
        };

        // List of available commands.
        let cmds = [$(stringify!($cmd)),+];

        // If the callee is not a valid command, get the command from the next argument.
        if !cmds.contains(&cmd.as_str()) {
            // Advance the iterator.
            let _ = args.next();

            let Some(cmd_arg) = args.peek() else {
                eprintln!("No utility was specified");
                exit(2);
            };

            cmd = cmd_arg.to_string_lossy().to_string();
        }

        let ret = match cmd.as_str() {
            $(
                stringify!($cmd) => $cmd(args),
            )+
            _ => {
                eprintln!("'{cmd}' utility not found");
                1
            }
        };

        exit(ret);
    }};
}

fn main() {
    setup!(basename, cat)
}
