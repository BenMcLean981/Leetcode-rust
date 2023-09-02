use crate::pkg::median_sorted_arrays;

#[allow(dead_code)]
mod pkg;

fn main() {
    let answer = median_sorted_arrays::Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]);

    println!("{:?}", answer);
}

