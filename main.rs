use std::io;
use std::cmp;
use std::time;
use time::Instant;

fn scoreinput(phrase: &str, input: &str) -> f32 {
    let mut correct: i32 = 0;
    let input_chars: Vec<char> = input.chars().collect();
    let phrase_chars: Vec<char> = phrase.chars().collect();
    let num_chars = cmp::min(input_chars.len(), phrase_chars.len());
    println!("{} {}", input_chars.len(), phrase_chars.len());
    for i in 0..num_chars {
        if input_chars[i] == phrase_chars[i] {
            correct += 1;
        }
    }
    return 100.0 * (correct as f32) / (input_chars.len() as f32)
}

fn main() {
    let phrase: &str = "the quick brown fox jumps over the lazy dog";
    let mut input = String::new();
    let start_t = Instant::now();
    println!("{}", phrase);
    io::stdin().read_line(&mut input).expect("");
    let elapsed_t = start_t.elapsed().as_millis();
    let input: &str = input.as_str().trim();
    println!("{} {}", (input.len() as f32 / 5.0) / (elapsed_t as f32 / 60000.0), scoreinput(phrase, input));
}
