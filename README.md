# Krustelkiste

A (partial) implementation of the POSIX Base Specification, Issue 7, utilities. Issue 7 because IEEE doesn't grant me access to Issue 8 through my university.

The application is supposed to work similar to [BusyBox](https://busybox.net), as that it is a single application binary that can invoke different utilities by either symlinking it as the utility's name or calling the binary directly with the utility's name as first argument.

> The word "Krustelkiste" is a German noun describing an unorganized box with all kinds of tools and items.

## Currently implemented
- basename
- cat
- cksum
- cmp

## Development
Each utility must be in a file with the same name as the utility, and expose one public function with the utility's
name. In `main.rs` just add the utility's name to the `setup!` macro invocation.

Use `Clap` to do command line argument parsing for the command and as a help utility for command usage. The comments
can be directly copied from the POSIX Base Specification. Checkout `cat.rs` as an example.

## Sources
- https://standards.ieee.org/ieee/1003.1/7101/
- https://www.gnu.org/software/coreutils/manual/coreutils.html
