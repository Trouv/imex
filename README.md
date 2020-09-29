# IMEx
[![Latest version](https://img.shields.io/crates/v/imex.svg)](https://crates.io/crates/imex)
[![Documentation](https://docs.rs/imex/badge.svg)](https://docs.rs/imex)

A library/CLI for merging multiple iterators/files into one, with the
optional use of an IMEx, or Iterator-Merging-Expression, for controlling the
merge.

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

## IMEx as a CLI tool
Using the CLI tool allows you to merge multiple files line-by-line.
To respect the Unix philosophy, stdin can also be merged with other files, and
the result is printed to the screen.

### Usage
Files are provided as positional arguments, and an IMEx can be provided using
the `-i` option.
The digits of the IMEx will refer to one of the files you provide in the order
you provide them, 0-indexed.
The filename `-` is reserved for stdin.

So, the following command will merge the output of the `ls` command with two
other files, taking 10 lines from stdin and then one from each of the files
until they're all exhausted, then output it to a file called `out.txt`:
```
$ ls | imex - file1.txt file2.txt -i "(0{10}12)*" > out.txt
```

### Installation
If you are a rust developer, you can install IMEx through cargo:
```
$ cargo install imex
```

Currently, imex doesn't exist in any other package manager.

## IMEx as a library
Using the IMEx crate in your code primarily gives you access to some new
functions on iterators that merge and return iterators. These can all be used
in a typical iterator processing chain. Details on usage and implementation can
be read in the crate's [documentation](https://docs.rs/imex/).

## Planned Functionality
There is one main feature planned for imex:
 * An optional interactive mode in the CLI to edit the IMEx and see results in
   real time, along with a peek into the upcoming lines of the files being
   merged.

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.
