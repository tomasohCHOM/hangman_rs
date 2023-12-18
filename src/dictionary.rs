extern crate rand;

use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const EASIES_FILENAME: &str = "words/easies.txt";
const MEDIUMS_FILENAME: &str = "words/mediums.txt";
const HARDS_FILENAME: &str = "words/hards.txt";

pub fn get_random_word(difficulty: char) -> String {
    let file_path = match difficulty {
        'e' => EASIES_FILENAME,
        'm' => MEDIUMS_FILENAME,
        'h' => HARDS_FILENAME,
        _ => "",
    };

    let f =
        File::open(file_path).unwrap_or_else(|e| panic!("File not found: {}: {}", file_path, e));
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    return lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
}
