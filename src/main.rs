use imex::IMExMerges;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use clap::{crate_authors, crate_version, App, Arg};

fn main() {
    let matches = App::new("imex")
        .about(
            "
Merge multiple files into one line-by-line, with the optional use of an IMEx,
or Iterator-Merging-Expression, for controlling the merge.

Documentation for writing an imex can be found at https://docs.rs/crate/imex

If stdin has data, the 0th index in the imex will refer to stdin.",
        )
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("files")
                .help("Paths of files to be merged. Maximum of 10.")
                .required(true)
                .max_values(10)
                .index(1),
        )
        .arg(
            Arg::with_name("imex")
                .help(
                    "Define imex to control the merge.
Defaults to (012...x)* where x is the
number of files provided minus one.",
                )
                .short("i")
                .long("imex")
                .takes_value(true),
        )
        .get_matches();

    let mut vec_lines: Vec<Lines<BufReader<File>>> = matches
        .values_of("files")
        .unwrap()
        .map(|path| BufReader::new(File::open(Path::new(path)).unwrap()).lines())
        .collect();

    let first = vec_lines.remove(0);

    let imex = match matches.value_of("imex") {
        Some(imex) => first.imex_merge_all(&mut vec_lines, imex).unwrap(),
        None => first.rot_merge_all(&mut vec_lines),
    };

    for line in imex {
        println!("{}", line.unwrap())
    }
}
