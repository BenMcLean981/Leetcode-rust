use crate::contests::biweekly::contest_112;

#[allow(dead_code)]
mod problems;
mod contests;

fn main() {
    let answer = contest_112::Solution::foo();

    println!("{:?}", answer);
}

