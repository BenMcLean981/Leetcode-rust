use crate::problems::is_subsequence;

#[allow(dead_code)]
mod problems;

fn main() {
    let answer = is_subsequence::Solution::is_subsequence(String::from("aaaaaa"), String::from("bbaaaa"));

    println!("{:?}", answer);
}

