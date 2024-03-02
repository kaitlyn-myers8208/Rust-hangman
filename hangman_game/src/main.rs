use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::env;
use std::fs;
// use std::fs::File;
// use std::io::prelude;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    println!("This is my Hangman game!");

    let rand_num = rand::thread_rng().gen_range(1..=828);
    println!("Random number is: {rand_num}");

    let words = lines_from_file("words.txt");
    // for line in words {
    //     println!("{:?}", line);
    // }
    // let mut word = String::new();
    // let word = words[rand_num];
    
    if let Some(word) = words.get(rand_num - 1) {
        println!("Random word: {}", word);
    } else {
        println!("Failed to get word at index {}", rand_num);
    }
    
    
    // let mut file = File::open("words.txt");
    
    // display_word(word);
}

fn display_word(word: String) {
    println!("The word is {word}");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


// write function that puts the underscores for length of word
// word class that has all the info about a word and displays the word
// user guesses
