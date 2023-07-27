use std::{
    fs::File,
    io,
    io::{BufReader, BufRead}
};

pub fn read_lines(file_name: String) -> io::Lines<BufReader<File>> {
    BufReader::new(
        File::open(&*file_name).unwrap()
    ).lines()
}
