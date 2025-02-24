use std::io;
use std::time::Instant;
use std::env;
use std::fs;

fn flush() {
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}

fn read_file(output: &mut String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let file_path = &args[1];
    *output = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
}

fn nth_word(string: &String, n: usize) -> &str {
    match string.split_whitespace().nth(n) {
        Some(w) => w,
        None => "",
    }
}

fn find_errors(original: &String, text: &String) -> usize {
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
    errors
}

fn main() {
    let stdin = io::stdin();
    let mut discard_input = String::new();
    let mut original = String::new();
    read_file(&mut original);
    println!();
    println!("{}", original);

    let mut text = String::new();
    print!("Type this text as fast as you can! (press enter to start):");
    flush();
    stdin.read_line(&mut discard_input).expect("input failed!");
    println!();
    let now = Instant::now();
    for _ in original.lines() {
        stdin.read_line(&mut text).expect("input failed!");
    }

    let elapsed = now.elapsed().as_millis();
    let length = text.chars().count() - 2;
    let speed = length * 12000 / (elapsed as usize);

    println!();
    println!("Errors:");
    let errors = find_errors(&original, &text);

    println!("Total: {}", errors);
    println!();
    println!("Length: {}", length);
    println!("Elapsed: {} ms", elapsed);
    println!("Speed: {} wpm", speed);
}
