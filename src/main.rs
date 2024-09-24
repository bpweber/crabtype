use std::io;
use std::cmp;
use std::time;
use time::Instant;
use std::fs;
use rand::Rng;

fn readfromfile() -> String {
    let words = fs::read_to_string("words.txt").expect("");
    return words;
}

fn generatephrase() -> String {
    let words = readfromfile();
    let words: Vec<_> = words.split("\n").collect();
    let mut rng = rand::thread_rng();
    let wordslen = words.len() - 1;
    let mut phrase = String::new();
    for _ in 0..10 {
        phrase = phrase + words[rng.gen_range(0..wordslen)] + " ";
    }
    return phrase; 
}

fn scoreinput(phrase: &str, input: &str) -> f32 {
    let mut correct: i32 = 0;
    let input_chars: Vec<char> = input.chars().collect();
    let phrase_chars: Vec<char> = phrase.chars().collect();
    let num_chars = cmp::min(input_chars.len(), phrase_chars.len());
    for i in 0..num_chars {
        if input_chars[i] == phrase_chars[i] {
            correct += 1;
        }
    }
    return (correct as f32) / (input_chars.len() as f32)
}

fn main() {
    let phrase: &str = &generatephrase();
    let mut input = String::new();
    let start_t = Instant::now();
    println!("{}", phrase);
    io::stdin().read_line(&mut input).expect("");
    let elapsed_t = start_t.elapsed().as_millis();
    let input: &str = input.as_str().trim();
    let wpm_raw = (input.len() as f32 / 5.0) / (elapsed_t as f32 / 60000.0);
    let accuracy = scoreinput(phrase, input);
    println!("{}% | {}wpm | {}raw", (100.0 * accuracy) as i32, (accuracy * wpm_raw) as i32, wpm_raw as i32);
}

