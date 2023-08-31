#[allow(dead_code)]
mod pkg;

use rand::{thread_rng, distributions::Alphanumeric, Rng};

use crate::pkg::longest_substring_without_repeating_characters;

fn main() {
    let s = String::from(make_test_case(10000));
    let answer = longest_substring_without_repeating_characters::Solution::length_of_longest_substring(s);

    println!("{}", answer);
}

fn make_test_case(len: usize) -> String {
    return thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();
}