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
    let mut first_text = String::new();
    read_file(&mut first_text);
    println!("{}", first_text);

    let mut second_text = String::new();
    print!("Now retype this text as fast as you can! (press enter to start):");
    flush();
    stdin.read_line(&mut discard_input).expect("input failed!");
    let now = Instant::now();
    for _ in first_text.lines() {
        print!("> ");
        flush();
        stdin.read_line(&mut second_text).expect("input failed!");
    }

    let elapsed = now.elapsed().as_millis();
    let length = second_text.chars().count() - 2;
    let speed = length * 12000 / (elapsed as usize);
    println!();

    if first_text == second_text {
        println!("All correct!");
    } else {
        println!("Incorrect.");
    }
    println!("Length: {}", length);
    println!("Elapsed: {} ms", elapsed);
    println!("Speed: {} wpm", speed);
}
