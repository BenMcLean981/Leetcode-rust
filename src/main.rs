#[allow(dead_code)]
mod pkg;

use crate::pkg::longest_substring_without_repeating_characters;

fn main() {
    let s = String::from("aab");
    let answer = longest_substring_without_repeating_characters::Solution::length_of_longest_substring(s);

    println!("{}", answer);
}

