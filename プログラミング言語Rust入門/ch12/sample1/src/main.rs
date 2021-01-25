use std::fs;
use std::fs::*;

use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
fn main() {
    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = std::fs::read_to_string(path) {
        println!("data is {}", data);
    }

    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = fs::read_to_string(path) {
        println!("data is {}", data);
    }

    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = read_to_string(path) {
        println!("data is {}", data);
    }

    let path = "unknown.txt";
    println!("read all lines.");
    if let Ok(data) = fs::read_to_string(path) {
        println!("data is {}", data);
    } else {
        println!("cannot open {}", path);
    }

    let path = "unknown.txt";
    println!("read all lines.");
    match fs::read_to_string(path) {
        Ok(data) => { println!("data is {}", data) },
        _ => { println!("cannot open {}", path) }
    }

    let path = "sample.txt";
    println!("read all lines by buffer.");
    let mut file = File::open(path)
        .expect("file not found.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("read error");
    println!("data is {}", data);

    let path = "sample.txt";
    println!("read all lines by buffer.");
    if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            println!("data is {}", data);
        }
    }

    let path = "sample.txt";
    println!("read 16 bytes by buffer.");
    let mut file = File::open(path)
        .expect("fie not found");
    let mut buf: [u8; 1] = [0; 1];
    for i in 0..16 {
        file.read(&mut buf);
        println!("buf is {}: {}", i, buf[0] as char);
    }

    println!("read every one line.");
    let file = File::open(path)
        .expect("file not found");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            println!("line is {}", l);
        }
    }
}
