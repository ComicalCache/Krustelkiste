# Krustelkiste

A (partial) implementation of the POSIX Base Specification, Issue 7, utilities. Issue 7 because IEEE doesn't grant me access to Issue 8 through my university.

The application is supposed to work similar to [BusyBox](https://busybox.net), as that it is a single application binary that can invoke different utilities by either symlinking it as the specified name or calling the binary directly with the utility as first argument.

> The word "Krustelkiste" is a German noun describing an unorganized box with all kinds of tools and items.

## Currently implemented
- basename
- cat
- cksum
- cmp

## Sources
- https://standards.ieee.org/ieee/1003.1/7101/
- https://www.gnu.org/software/coreutils/manual/coreutils.html
