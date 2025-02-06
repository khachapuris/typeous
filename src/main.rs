use std::io;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Text: {}", input_text);
}
