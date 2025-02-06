use std::io;
use std::time::Instant;

fn main() {
    let text = "The quick brown fox jumps over the lazy dog.";
    println!("Type this text as fast as you can (press enter to start):\n");
    println!("> {}", text);
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let now = Instant::now();

    io::stdin().read_line(&mut input_text).unwrap();
    input_text.remove(0);
    input_text.pop();

    let elapsed = now.elapsed().as_millis();
    let length = input_text.chars().count();
    let speed = length * 12000 / (elapsed as usize);
    println!();
    if text == input_text {
        println!("All correct!");
    } else {
        println!("Incorrect.");
    }
    println!("Length: {}", length);
    println!("Elapsed: {} ms", elapsed);
    println!("Speed: {} wpm", speed);
}
