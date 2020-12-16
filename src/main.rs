use fastqfilt::*;
use std::env;
use std::process::exit;
use std::str::FromStr;

fn main() {
    let config = Config::new();
    foo(&config.inpath, &config.outpath, config.min_qual, config.min_len, config.max_len, config.phred_offset)
}

struct Config{
    inpath: String,
    outpath: String,
    min_qual: f64,
    min_len: usize,
    max_len: usize,
    phred_offset: u8,
}

impl Config {
    fn new() -> Config {
        let arguments: Vec<String> = env::args().collect();
        if arguments.len() < 6 + 1 {
            eprintln!("Usage: nanofilt <inpath> <outpath> <minqual> <minlength> <maxlength> <phred_offset>");
            exit(1);
        }
        let min_qual: f64 = parse_num::<f64>(&arguments[3]);
        let min_len: usize = parse_num::<usize>(&arguments[4]);
        let max_len: usize = parse_num::<usize>(&arguments[5]);
        let phred_offset: u8 = parse_num::<u8>(&arguments[6]);
        Config{inpath: arguments[1].clone(), outpath: arguments[2].clone(),
        min_qual, min_len, max_len, phred_offset}
    }
}

fn parse_num<T: FromStr>(str: &str) -> T 
where <T as FromStr>::Err: std::fmt::Display {
    match str.parse::<T>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Cannot parse argument {} as the right number type: {}", str, e);
            exit(1)
        }
    }
}
