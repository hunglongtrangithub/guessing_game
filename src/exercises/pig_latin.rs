use crate::utils;
use regex::Regex;

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = "aiueo";
    let first_char = match word.chars().next() {
        Some(c) => c,
        // empty string
        None => return word.to_string(),
    };
    if vowels.contains(first_char) {
        // 1. If a word starts with a vowel, add "hay" to the end of the word
        format!("{}hay", word)
    } else {
        // 2. If a word starts with a consonant, move the consonant to the end of the word and add "ay"
        format!("{}{}ay", &word[1..], first_char)
    }
}

pub fn launch() {
    utils::clear_screen();
    let re = Regex::new(r"[a-zA-Z]+").expect("Failed to compile regex. You dumbass, wrong regex!");
    loop {
        println!("Enter sentence to convert to Pig Latin, or press Enter to go back");
        let sentence = utils::read_input();
        if sentence.trim().is_empty() {
            break;
        }

        let replacement = |caps: &regex::Captures| {
            let word = caps.get(0).unwrap().as_str();
            convert_to_pig_latin(word)
        };

        let result = re.replace_all(&sentence, &replacement);
        println!("Pig Latin: {}", result);
    }
}
