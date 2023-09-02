use crate::pkg::is_subsequence;

#[allow(dead_code)]
mod pkg;

fn main() {
    let answer = is_subsequence::Solution::is_subsequence(String::from("aaaaaa"), String::from("bbaaaa"));

    println!("{:?}", answer);
}

