use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::env;
use std::fs;
use std::collections::HashSet;
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

    // let mut wordVec: Vec<char> = Vec::new();   
    // for letter in word.chars() {
    //     wordVec.push('_');
    // }
    let mut wordVec: Vec<char> = vec!['_'; word.len()];
    let mut correct_guesses: HashSet<char> = HashSet::new();
    let mut guesses_remaining = 5;
    
    loop {
        println!("Guess a letter: ({} guesses remaining)", guesses_remaining);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: char = match guess.trim().parse() {
        //     Ok(guess) => guess,
        //     Err(_) => continue,
        // };
        let guess: char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };  

        if correct_guesses.contains(&guess) {
            println!("You already guessed '{}'", guess);
            continue;
        }

        if word.contains(guess) {
            println!("'{}' found. Good job!", guess);
            correct_guesses.insert(guess);

            for (index, letter) in word.chars().enumerate() {
                if letter == guess {
                    wordVec[index] = guess;
                }
            }
        } else {
            println!("'{}' not found", guess);
            guesses_remaining -= 1;
        }

        if wordVec.iter().all(|&c| c != '_') {
            println!("Congratulations! You guessed the word: {}", word);
            break;
        }

        for letter in &wordVec {
            print!("{} ", letter);
        }
        println!();

        if guesses_remaining == 0 {
            println!("Out of guesses. The word was: {}", word);
            break;
        }

        // TO DO
        // create a list of letters that we can compare to so you don't reset the letters every time
        // add the bad letters to print out
        // give them only 5 guess or so
        // make a win statement

        
        // display_word(&word, guess, wordVec.clone());
    }
}

fn display_word(word: &String, guess: char, mut wordVec: Vec<char>) {

    // for letter in word.chars() {
    //     // println!("This {letter}");
    //     if letter == guess {
    //         letters.push(letter);
    //         // print!("{letter}");
    //     } else {
    //         if letter.is_alphabetic() {
    //             letters.push(letter);
    //         } else {
    //             letters.push('_');
    //             // print!("_")
    //         }
    //     }
    //     // match letter {
    //     //     {guess} => print!("{letter}"),
    //     //     _ => print!("_"),
    //     // };
    //     // println!("_");
    // }
    // println!();
    // println!("The word is {word}");
    
    
    // match word.find(guess) {
        //     Some(index) => println!("Found {} at index {}", guess, index),
        //     None => println!("Character '{}' not found", guess),
        // }
        
    match word.find(guess) {
        Some(index) => {
            println!("'{}' found. Good job!", guess);
            
            if let Some(letter) = wordVec.get_mut(index) {
                *letter = guess;
            }
        },
        None => println!("'{}' not found", guess),
    }
    
    for letter in wordVec {
        // println!("Printing out the word");
        print!("{letter}");
    }
        
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

fn get_word() {
    let rand_num = rand::thread_rng().gen_range(1..=828);
    // println!("Random number is: {rand_num}");
    let words = lines_from_file("words.txt");

    if let Some(word) = words.get(rand_num - 1) {
        // println!("Random word: {}", word);
        // display_word(word);
    } else {
        println!("Failed to get word at index {}", rand_num);
    }
}


// write function that puts the underscores for length of word
// word class that has all the info about a word and displays the word
// user guesses
