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
    let mut errors = 0;
    println!();
    let mut b = 0;
    for a in 0..text.split_whitespace().count() {
        let original_word = original.split_whitespace().nth(b);
        let original_next = original.split_whitespace().nth(b + 1);
        let word = text.split_whitespace().nth(a);
        let word_next = text.split_whitespace().nth(a + 1);
        if word == original_word {
            // Everything is ok
        } else if word_next == original_next {
            println!("{} != {}", word.unwrap(), original_word.unwrap());
            // There is a typo in the word
            errors += 1;
        } else if word == original_next {
            println!("- {}", original_word.unwrap());
            errors += 1;
            b += 1;
        } else if word_next == original_word {
            println!("+ {}", word.unwrap());
            errors += 1;
            b -= 1;
        } else {
            errors += 1;
        };
        b += 1;
    }

    println!("Errors: {}", errors);
    println!("Length: {}", length);
    println!("Elapsed: {} ms", elapsed);
    println!("Speed: {} wpm", speed);
}
