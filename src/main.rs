use std::io;
use std::cmp;
use std::time;
use std::fs;
use std::env;
use time::Instant;
use rand::Rng;

fn readfromfile() -> String {
    let words = fs::read_to_string("words.txt").expect("");
    return words;
}

fn generatephrase(diff: i32, plen: i32) -> String {
    let words = readfromfile();
    let words: Vec<_> = words.split("\n").collect();
    let mut rng = rand::thread_rng();
    let wordslen = ((diff as f32 / 10.0) * words.len() as f32) as usize - 1;
    let mut phrase = String::new();
    for _ in 0..plen {
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

fn parseargs(args: Vec<String>) -> Vec<i32> {
    let mut diff: i32 = 10;
    let mut plen: i32 = 15;
    if args.len() > 1 {
        diff = match args[1].parse() {
            Ok(n) => cmp::min(n, diff),
            Err(_) => {
                println!("Illegal argument!");
                diff
            }, 
        };
    }
    if args.len() > 2 {
        plen = match args[2].parse() {
            Ok(n) => cmp::min(n, plen),
            Err(_) => {
                println!("Illegal argument!");
                plen
            }, 
        };
    }
    return vec![diff, plen]
}

fn main() {
    let args: Vec<i32> = parseargs(env::args().collect());
    let diff = args[0];
    let plen = args[1];
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{0} {1}", diff, plen);
    loop {
        let phrase: &str = &generatephrase(diff, plen);
        let mut input = String::new();
        let start_t = Instant::now();
        println!("{}", phrase);
        io::stdin().read_line(&mut input).expect("");
        let elapsed_t = start_t.elapsed().as_millis();
        let input: &str = input.as_str().trim();
        let wpm_raw = (input.len() as f32 / 5.0) / (elapsed_t as f32 / 60000.0);
        let accuracy = scoreinput(phrase, input);
        println!("{}% | {}wpm | {}raw\n", (100.0 * accuracy) as i32, (accuracy * wpm_raw) as i32, wpm_raw as i32);
    }
}

