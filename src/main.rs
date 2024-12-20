use core::panic;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use regex::Regex;
use walkdir::WalkDir;

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

    let mut re;
    let mut filename;

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        re = Regex::new(arg).unwrap();

        for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }

            filename = entry.file_name().to_string_lossy();
            if re.is_match(&filename) {
                println!("Converting: {}", filename);
                convert(&filename);
            }
        }
    }
}
