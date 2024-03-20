use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::env;
use std::fs;
use std::collections::HashSet;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    println!("This is my Hangman game!");

    let mut word = String::new();
    word = get_word();

    let mut wordVec: Vec<char> = vec!['_'; word.len()];
    let mut correct_guesses: HashSet<char> = HashSet::new();
    let mut guesses_remaining = 8;

    println!("Rules: ");
    println!("1. You have 8 guesses to guess the word");
    println!("2. Duplicate letters don't count as a guess");
    println!("3. Guess letters until you lose or guess the word");
    println!("4. Have fun!");
    println!("Now, let's go!");
    println!();
    
    loop {
        println!("Guess a letter: ({} guesses remaining)", guesses_remaining);

        for letter in &wordVec {
            print!("{} ", letter);
        }
        println!();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };  

        if correct_guesses.contains(&guess) {
            println!("You already guessed '{}'", guess);
            continue;
        }

        if guess == '*' {
            guesses_remaining = 9;
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
    }
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()    
}

fn get_word() -> String {
    let rand_num = rand::thread_rng().gen_range(1..=828);
    // println!("Random number is: {rand_num}");
    let words = lines_from_file("words.txt");
    
    if let Some(word) = words.get(rand_num - 1) {
        // println!("Random word: {}", word);
        let mut random_word = String::new();
        random_word = word.to_string();
        return random_word;
    } else {
        println!("Failed to get word at index {}", rand_num);
        return "face".to_string();
    }
}

// fn display_word(word: &String, guess: char, mut wordVec: Vec<char>) {

//     // for letter in word.chars() {
//     //     // println!("This {letter}");
//     //     if letter == guess {
//     //         letters.push(letter);
//     //         // print!("{letter}");
//     //     } else {
//     //         if letter.is_alphabetic() {
//     //             letters.push(letter);
//     //         } else {
//     //             letters.push('_');
//     //             // print!("_")
//     //         }
//     //     }
//     //     // match letter {
//     //     //     {guess} => print!("{letter}"),
//     //     //     _ => print!("_"),
//     //     // };
//     //     // println!("_");
//     // }
//     // println!();
//     // println!("The word is {word}");
    
    
//     // match word.find(guess) {
//         //     Some(index) => println!("Found {} at index {}", guess, index),
//         //     None => println!("Character '{}' not found", guess),
//         // }
        
//     match word.find(guess) {
//         Some(index) => {
//             println!("'{}' found. Good job!", guess);
            
//             if let Some(letter) = wordVec.get_mut(index) {
//                 *letter = guess;
//             }
//         },
//         None => println!("'{}' not found", guess),
//     }
    
//     for letter in wordVec {
//         // println!("Printing out the word");
//         print!("{letter}");
//     }
        
// }