use crate::contests::biweekly::contest_112::max_sum;

#[allow(dead_code)]
mod problems;
mod contests;

fn main() {
    let answer = max_sum::Solution::max_sum(vec![1, 2, 1, 2], 2, 4);

    println!("{:?}", answer);
}

