use crate::utils;

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
    loop {
        println!("Enter a word to convert to Pig Latin, or press Enter to go back");
        let word = utils::read_input();
        if word.trim().is_empty() {
            break;
        }
        let word = word.trim();
        let pig_latin = convert_to_pig_latin(word);
        println!("Pig Latin: {}", pig_latin);
    }
}
