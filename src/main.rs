use core::panic;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn read_lines<'a>(file: &'a File) -> io::Result<Vec<String>> {
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn write_lines(file: &mut File, lines: Vec<String>) -> io::Result<()> {
    for line in lines.into_iter() {
        let converted_line = line + "\n";
        let _ = file.write_all(converted_line.as_bytes());
    }
    Ok(())
}

fn convert(filename: &str) {
    let file = match OpenOptions::new().read(true).open(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file: {:?}", error);
        }
    };

    let lines = match read_lines(&file) {
        Ok(lines) => lines,
        Err(error) => {
            panic!("Error reading lines from file: {:?}", error);
        }
    };

    let mut file = match OpenOptions::new().write(true).truncate(true).open(filename) {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file: {:?}", error);
        }
    };

    if let Err(e) = write_lines(&mut file, lines) {
        panic!("Error writing to file: {:?}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for (_, arg) in args.iter().enumerate() {
        if !arg.contains(".txt") {
            continue;
        }
        println!("Converting: {}", arg);
        convert(arg);
    }
}
