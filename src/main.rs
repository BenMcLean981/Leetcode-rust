#[allow(dead_code)]
mod pkg;

use crate::pkg::add_two_numbers::{self, vec_to_linked_list, make_vector};

fn main() {
    let l1 = vec_to_linked_list(vec![9,9,9,9,9,9,9]);
    let l2 = vec_to_linked_list(vec![9, 9, 9, 9]);

    let answer = add_two_numbers::Solution::add_two_numbers(l1, l2);
    let answer_vec = make_vector(answer);

    println!("{:?}", answer_vec);
}

