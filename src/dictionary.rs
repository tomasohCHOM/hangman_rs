extern crate rand;

use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "words/words.txt";

pub fn get_random_word() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    return lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
}
