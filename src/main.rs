use std::io;
use std::time::Instant;
use std::process;
use std::env;
use std::fs;

use serde::Deserialize;
use serde_yaml::{self};

#[derive(Debug, Deserialize)]
struct Config {
    statistics: Vec<String>,
}

/// Get the file name from the command line arguments
/// and return the contents of the file
fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "typefile.txt";
    if args.len() >= 2 {
        file_path = &args[1];
    };
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

fn load_config() -> Config {
    let file = fs::File::open("config.yaml")
        .expect("Could not open the config file.");
    serde_yaml::from_reader(file).expect("Could not read config.")
}

fn play(original: &String) -> (String, usize) {
    let stdin = io::stdin();
    let mut discard_input = String::new();
    let mut text = String::new();

    println!();
    println!("{}", original);
    println!("Type this text as fast as you can! (press enter to start):");
    stdin.read_line(&mut discard_input).expect("input failed!");
    let now = Instant::now();
    for _ in original.lines() {
        stdin.read_line(&mut text).expect("input failed!");
    };
    let elapsed = now.elapsed().as_millis();
    (text, elapsed as usize)
}

/// Return the nth word of the string if it exists;
/// otherwise return an empty string
fn nth_word(string: &String, n: usize) -> &str {
    match string.split_whitespace().nth(n) {
        Some(w) => w,
        None => "",
    }
}

fn print_errors(original: &String, text: &String) {

    println!();
    println!("Errors");
    println!("------");

    let mut errors = 0;
    let mut b = 0;
    for a in 0..text.split_whitespace().count() {
        let original_word = nth_word(original, b);
        let original_next = nth_word(original, b + 1);
        let word = nth_word(text, a);
        let word_next = nth_word(text, a + 1);
        if word == original_word {
            // Everything is ok
        } else if word_next == original_next {
            // There is a typo in the word
            println!("{} != {}", word, original_word);
            errors += 1;
        } else if word == original_next {
            println!("- {}", original_word);
            errors += 1;
            b += 1;
        } else if word_next == original_word {
            println!("+ {}", word);
            errors += 1;
            b -= 1;
        } else {
            println!("{} != {}", word, original_word);
            errors += 1;
        };
        b += 1;
    };
    println!("Total: {}", errors);
}

fn print_stats(text: &String, elapsed: usize, config: &Config) {
    let length = text.chars().count() - 2;
    let wpm = length * 12000 / (elapsed as usize);
    let cpm = length * 60000 / (elapsed as usize);
    println!();
    println!("Statistics");
    println!("----------");
    for option in &config.statistics {
        match option.as_str() {
            "chars" => println!("Characters: {}", length),
            "words" => println!("Words: {}", text.split_whitespace().count()),
            "millis" => println!("Elapsed: {} s", elapsed / 1000),
            "seconds" => println!("Elapsed: {} ms", elapsed),
            "cpm" => println!("Speed: {} cpm", cpm),
            "wpm" => println!("Speed: {} wpm", wpm),
            option => println!("Unknown option: {}", option),
        }
    }
}

fn main() {
    let config = load_config();
    println!("{:?}", config);
    let original = read_file();
    let (text, elapsed) = play(&original);
    print_errors(&original, &text);
    print_stats(&text, elapsed, &config);
}
