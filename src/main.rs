use std::io;
use std::time::Instant;

fn main() {
    let text = "Never start typing before the time starts.";
    println!("Type this text as fast as you can:");
    println!("> {}", text);
    println!("Press Enter to start:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let now = Instant::now();

    io::stdin().read_line(&mut input_text).unwrap();

    let elapsed = now.elapsed().as_millis();
    let length = input_text.chars().count();
    let speed = length * 12000 / (elapsed as usize);
    println!("Length: {}", length);
    println!("Elapsed: {} ms", elapsed);
    println!("Speed: {} wpm", speed);
}
