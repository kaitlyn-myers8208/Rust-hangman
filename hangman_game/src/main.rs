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

    // get_word();
    let mut word = String::new();
    word = "face".to_string(); 
    let mut letters: Vec<char> = Vec::new();   
    for letter in word.chars() {
        letters.push('_');
    }
    
    loop {
        println!("Guess a letter: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: char = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };    

        // TO DO
        // create a list of letters that we can compare to so you don't reset the letters every time
        // add the bad letters to print out
        // give them only 5 guess or so
        // make a win statement




        // let guess: char = guess.trim().parse().expect("Please type a letter!");
        // let mut file = File::open("words.txt");
        
        display_word(&word, guess, letters.clone());
    }
}

fn display_word(word: &String, guess: char, mut letters: Vec<char>) {
    // let length = word.len();
    // let letters = 
    // let user_guess = guess;
    for letter in word.chars() {
        // println!("This {letter}");
        if letter == guess {
            letters.push(letter);
            // print!("{letter}");
        } else {
            if letter.is_alphabetic() {
                letters.push(letter);
            } else {
                letters.push('_');
                // print!("_")
            }
        }
        // match letter {
        //     {guess} => print!("{letter}"),
        //     _ => print!("_"),
        // };
        // println!("_");
    }
    for letter in letters {
        // println!("Printing out the word");
        print!("{letter}");
    }
    println!();
    println!("The word is {word}");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn populateList() {

}

// fn get_word() {
//     let rand_num = rand::thread_rng().gen_range(1..=828);
//     // println!("Random number is: {rand_num}");
//     let words = lines_from_file("words.txt");

//     if let Some(word) = words.get(rand_num - 1) {
//         // println!("Random word: {}", word);
//         display_word(word);
//     } else {
//         println!("Failed to get word at index {}", rand_num);
//     }
// }


// write function that puts the underscores for length of word
// word class that has all the info about a word and displays the word
// user guesses
