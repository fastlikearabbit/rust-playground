#![forbid(unsafe_code)]
use std::process;
use std::{fs::File, io::BufRead, io::BufReader};
use std::collections::HashSet;

fn read_lines_into_set(reader: BufReader<File>) -> HashSet<String> {
    let mut lines_set = HashSet::new();

    for line in reader.lines() {
        match line {
            Ok(line) => lines_set.insert(line),
            Err(err) => { eprintln!("{}", err); process::exit(-2); },
        };
    }
    lines_set
}
fn print_unique_lines(first: File, second: File) {
    let first_set: HashSet<String> = read_lines_into_set(BufReader::new(first));
    let second_set: HashSet<String> = read_lines_into_set(BufReader::new(second));

    for line in first_set.iter() {
        if second_set.contains(line) {
            println!("{}", line);
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: com <first_file> <second_file>");
        process::exit(-1);
    }

    // TODO: find a way to deal with file-related errors
    let first_file = File::open(&args[1]).unwrap();
    let second_file = File::open(&args[2]).unwrap();
    print_unique_lines(first_file, second_file);
}
