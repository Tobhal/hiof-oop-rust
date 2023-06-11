use std::fs::File;
use std::io;
use std::io::{
    BufRead,
    BufReader
};

pub fn read_lines(file_name: String) -> io::Lines<BufReader<File>> {
    BufReader::new(
        File::open("csv/".to_string() + &*file_name).unwrap()
    ).lines()
}
