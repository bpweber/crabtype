use std::io;

fn scoreinput(phrase: &str, input: &str) -> i32 {
    let mut correct: i32 = 0;
    let input_chars: Vec<char> = input.chars().collect();
    let phrase_chars: Vec<char> = phrase.chars().collect();
    for (i, c) in phrase_chars.iter().enumerate() {
        if *c == input_chars[i] {
            correct += 1;
        }
    }
    return correct
}

fn main() {
    let phrase: &str = "the quick brown fox jumps over the lazy dog";
    let mut input = String::new();
    println!("{}", phrase);
    io::stdin().read_line(&mut input).expect("");
    let input: &str = input.as_str().trim();
    println!("{} {}", (input == phrase), scoreinput(phrase, input));
//  println!("{}", input);
}
