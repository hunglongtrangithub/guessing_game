use crate::games::utils;
use convert_case::{Case, Casing};

pub fn christmas_lyrics() {
    utils::clear_screen();
    println!("Christmas Carol Lyrics: Twelve Days of Christmas\n");
    let combined = [
        ("first", "a", "partridge in a pear tree"),
        ("second", "two", "turtle doves"),
        ("third", "three", "French hens"),
        ("fourth", "four", "calling birds"),
        ("fifth", "five", "golden rings"),
        ("sixth", "six", "geese a-laying"),
        ("seventh", "seven", "swans a-swimming"),
        ("eighth", "eight", "maids a-milking"),
        ("ninth", "nine", "ladies dancing"),
        ("tenth", "ten", "lords a-leaping"),
        ("eleventh", "eleven", "pipers piping"),
        ("twelfth", "twelve", "drummers drumming"),
    ];
    for i in 0..combined.len() {
        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            combined[i].0
        );
        for j in (0..=i).rev() {
            let lyric = if j == 0 {
                if i == 0 {
                    format!("{} {}.", combined[j].1, combined[j].2)
                } else if i == combined.len() - 1 {
                    format!("and {} {}!", combined[j].1, combined[j].2)
                } else {
                    format!("and {} {}.", combined[j].1, combined[j].2)
                }
            } else {
                format!("{} {},", combined[j].1, combined[j].2)
            };
            println!("{}", lyric.to_case(Case::Sentence));
        }
        println!();
    }
}
