use imex::IMExMerges;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Lines};
use std::path::Path;

use clap::{crate_authors, crate_version, App, Arg};

fn main() {
    let matches = App::new("imex")
        .about(
            "
Merge multiple files into one line-by-line, with the optional use of an IMEx,
or Iterator-Merging-Expression, for controlling the merge. 

Documentation for writing an IMEx can be found at https://docs.rs/crate/imex
In this case, the digits of the IMEx will refer to one of the files you provide
in the order you provide them, 0-indexed.

The filename '-' is reserved for stdin.",
        )
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("FILES")
                .help("Paths of files to be merged. Maximum of 10.")
                .required(true)
                .max_values(10)
                .index(1),
        )
        .arg(
            Arg::with_name("IMEX")
                .help(
                    "Define IMEx to control the merge.
Defaults to (012...x)* where x is the
number of files provided minus one.",
                )
                .short("i")
                .long("imex")
                .takes_value(true),
        )
        .get_matches();

    let mut vec_lines: Vec<Lines<Box<dyn BufRead>>> = matches
        .values_of("FILES")
        .expect("Required argument is missing.")
        .map(|path| {
            let bufread: Box<dyn BufRead> = if path == "-" {
                Box::new(BufReader::new(stdin()))
            } else {
                Box::new(BufReader::new(match File::open(Path::new(path)) {
                    Ok(file) => file,
                    Err(_) => panic!("No such file or directory: {}", path),
                }))
            };
            bufread.lines()
        })
        .collect();

    let first = vec_lines.remove(0);

    let imex = match matches.value_of("IMEX") {
        Some(imex) => match first.imex_merge_all(&mut vec_lines, imex) {
            Ok(res) => res,
            Err(e) => panic!("Invalid IMEx\n{}", e),
        },
        None => first.rot_merge_all(&mut vec_lines),
    };

    for line in imex {
        println!("{}", line.unwrap())
    }
}
