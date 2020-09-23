use clap::{App, Arg};
use imex::IMExMerges;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let matches = App::new("imex")
        .arg(
            Arg::with_name("files")
                .required(true)
                .max_values(10)
                .index(1),
        )
        .arg(
            Arg::with_name("imex")
                .short("i")
                .long("imex")
                .takes_value(true),
        )
        .get_matches();

    let mut vec_lines: Vec<Lines<BufReader<File>>> = matches
        .values_of("files")
        .unwrap()
        .map(|x| BufReader::new(File::open(Path::new(x)).unwrap()).lines())
        .collect();

    let mut rest = vec_lines.split_off(1);
    let first = vec_lines.pop().unwrap();

    let imex = match matches.value_of("imex") {
        Some(imex) => first.imex_merge_all(&mut rest, imex).unwrap(),
        None => first.rot_merge_all(&mut rest),
    };

    for line in imex {
        println!("{}", line.unwrap())
    }
}
