use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => { // expected number of argument.
            println!("input file: {}, output file: {}", args[1], args[2]);
        },
        _ => {
            eprintln!("wrong number of argument!");
            std::process::exit(1);
        }
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("error reading file");
    println!("reading file: {}", filename);

    let token : Vec<&str> =  contents.split('\n').collect();
    let mut vec8 : Vec<u8> = Vec::new();
    for t in &token {
        let val64: u64 = match t.parse::<u64>() {
            Ok(n) => {
                n
            },
            Err(e) => {
                eprintln!("error: non-integer line \"{}\": {}", t, e);
                return;
            }
        };

        // uncomment this to print each line
        // println!("val64: {}", val64);

        let val8 = val64.to_le_bytes();
        for val in val8.iter(){
            vec8.push(*val);
        }
    }

    let out = &args[2];
    let mut file = File::create(out).unwrap();
    file.write_all(vec8.as_slice()).unwrap();
    println!("done writing to file {}", out);
}