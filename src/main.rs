use std::io;
use std::time::Instant;
use std::fs;
use dirs;
use std::env;

use serde::{Serialize, Deserialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    statistics: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config { statistics: vec![
            String::from("wpm"),
        ] }
    }
}

fn read_file() -> String {
    let mut text_path = dirs::home_dir()
        .expect("Did not find the home directory");
    text_path.push("typeme.txt");
    let text_path = text_path.as_path();
    if text_path.exists() {
        fs::read_to_string(text_path)
            .expect("Should have been able to read typeme.txt")
    } else {
        "Brown jars prevented the mixture
from freezing too quickly.\n".to_string()
    }
}

fn load_config() -> Config {
    let mut path = dirs::config_dir()
        .expect("Did not find the config directory");
    path.push("typeous");
    path.push("config.yaml");
    let path = path.as_path();
    if path.exists() {
        let content = fs::read_to_string(path)
            .expect("Should have been able to read the config file");
        serde_yaml::from_str(&content)
            .expect("Invalid YAML in configuration file")
    } else {
        let default_config = Config::default();
        let yaml = serde_yaml::to_string(&default_config).unwrap();
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        fs::write(path, yaml)
            .expect("Should have been able to create config file");
        default_config
    }
}

fn print_help() {
    let app_name = env::args().nth(0).unwrap();
    match env::args().nth(1) {
        Some(help) if help == String::from("-h")
        || help == String::from("--help") => {
            let mut path = dirs::config_dir()
                .expect("Did not find the config directory");
            path.push("typeous");
            path.push("config.yaml");
            let path = path.as_path();
            println!("Usage: {} [-h | --help]", app_name);
            println!("Practice touch typing with the text from typeme.txt");
            println!("in your home directory.");
            println!();
            println!("Help options:");
            println!("  -h, --help    display this help and exit");
            println!();
            println!("The configuration is stored in {}", path.display());

            std::process::exit(0);
        }
        Some(x) => {
            eprintln!("Unknown option: {}", x);
            eprintln!("See {} --help for a full list of options", app_name);
            std::process::exit(1);
        }
        None => {}
    }
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

    let original = original.replace('“', "\"")
        .replace('”', "\"")
        .replace('‘', "'")
        .replace('’', "'")
        .replace('—', "-")
        .replace('–', "-");
    let mut errors = 0;
    let mut b = 0;
    for a in 0..text.split_whitespace().count() {
        let original_word = nth_word(&original, b);
        let original_next = nth_word(&original, b + 1);
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
            "lines" => println!("Lines: {}", text.lines().count()),
            "seconds" => println!("Elapsed: {} s", elapsed / 1000),
            "millis" => println!("Elapsed: {} ms", elapsed),
            "cpm" => println!("Speed: {} cpm", cpm),
            "wpm" => println!("Speed: {} wpm", wpm),
            option => println!("Unknown option: {}", option),
        }
    }
}

fn main() {
    print_help();
    let config = load_config();
    let original = read_file();
    let (text, elapsed) = play(&original);
    print_errors(&original, &text);
    print_stats(&text, elapsed, &config);
}
