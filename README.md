# zprfly
zprfly is a (planned) cli tool for merging multiple files/streams into one, line by line.

## Usage
The most basic example would be `zprfly file1 file2`.
This output of this command would be the first line of `file1`, followed by the first line of `file2`, then the second line of `file1`, second line of `file2`, and so on.
In other words, it combines the files like a zipper fly interlaces the teeth of a zipper (hence the name).
Once one of the files is exhausted, the rest of the output would be the remainder of the other file.

If this is all that zprfly was capable of, then it would be redundant with `paste -d file1 file2`.
There are a couple more features that set zprfly apart.
One of these is the ability to write regex-like patterns using `--patern` or `-p`.
For example, `zprfly file1 file2 -p (122)*`, would still alternate lines from file1 and file2, except it would output 2 lines from file2 before returning to file1.
Here is a description of the pattern mechanisms that will be available in zprfly:
- Digits - indicates which file to consume a line from
- `()` - defines a group
- `*` - repeats the previous digit/group until the relevant files are exhausted
- `{x}` - repeats the previous digit/group `x` times

The other planned feature of zprfly is an optional curses interface (using `pancurses`) for writing patterns and seeing immediate results while also seeing the next few lines from the files you've selected.

## Status
Currently this project doesn't have any functionality, just planned functionality.
