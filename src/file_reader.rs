use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub(crate) fn read_lines(file_name: String) -> io::Lines<BufReader<File>> {
    let file = File::open("csv/".to_string() + &*file_name).unwrap();
    return io::BufReader::new(file).lines();
}
