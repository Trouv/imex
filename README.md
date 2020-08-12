# IMEx
A library for merging multiple iterators into one iterator, with the optional
use of an IMEx, or Iterator-Merging-Expression, for controlling the merge. imex
will also be a command line tool for merging files line-by-line in the future.

[![Latest version](https://img.shields.io/crates/v/imex.svg)](https://crates.io/crates/imex)
[![Documentation](https://docs.rs/imex/badge.svg)](https://docs.rs/imex)

## Writing an IMEx
IMEx is based off of RegEx. A quick reference of IMEx syntax:
 * Digits - indicates the index of the iterator to consume an item from
 * `()` - defines a group
 * `*` - repeats the previous digit/group until the relevant iterator\(s\) are
   exhausted
 * `{x}` - repeats the previous digit/group `x` times, or until the relevant
   iterator\(s\) are exhausted.

### Examples

`"0110"` on two iterators: results in a merged iterator that starts
with the first item of the first iterator, then the first two items of the
second iterator, then the second item of the first iterator

`"(01)*"` on two iterators: results in a merged iterator whose elements
alternate between the elements of the input iterators until they are both
exhausted.

`"(012){4}(122)*"` on three iterators: results in a merged iterator whose
elements rotate through the elements of the input iterators 4 times, then picks
an element of the second iterator followed by two elements of the third
repeatedly until they are both exhausted.

## IMEx as a library
Using the IMEx crate in your code primarily gives you access to some new
functions on iterators that merge and return iterators. These can all be used
in a typical iterator processing chain. Details on usage and implementation can
be read in the crate's [documentation](https://docs.rs/imex/).

## Planned Functionality
There are two main features planned for imex:
 * CLI tool possibly titled `fmex` for merging files line-by-line using an
   IMEx.  Will follow the UNIX philosophy, and will be able to merge files with
   stdin.
 * An optional curses TUI for this tool to edit the IMEx and see results in
   real time, along with a peek into the upcoming lines of the files being
   merged.

## License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
