use std::{collections::HashMap, cmp};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let map = make_indices_hashmap(s.clone());
        
        let length = s.len();
        let length = find_largest_distance(map, length).unwrap_or(length);
        
        return length as i32;
    }
}

// Map map of characters, and indices of those characters.
fn make_indices_hashmap(s: String) -> HashMap<char, Vec<usize>> {
    let mut result = HashMap::<char, Vec<usize>>::new();

    for (i, c) in s.chars().enumerate() {
        result.entry(c).or_insert(vec![]);
        result.entry(c).and_modify(|v| v.push(i));
    }

    return result;
}

// Look for longest distance between character occurances.
// If each character only occurs once (example, "abc") then this
// returns None
fn find_largest_distance(map: HashMap<char, Vec<usize>>, length: usize) -> Option<usize> {
    let result = map
        .values()
        .map(|v| find_largest_distance_within_vec(v, length))
        .max();

    return match result {
        Some(v) => v,
        None => None,
    }
}

// Take vec of character indices, find longest gap.
fn find_largest_distance_within_vec(v: &Vec<usize>, length: usize) -> Option<usize> {
    if v.len() == 0 {
        return None;
    } else if v.len() == 1 {
        let first = v[0];
        let last = v[v.len() - 1];
        
        let start_diff = first;
        let end_diff = length - last;

        return Some(cmp::max(start_diff, end_diff));
    } else {
        return v
            .iter()
            .zip(v.iter().skip(1))
            .map(|f| f.1 - f.0)
            .max();
    }
}

pub (crate) struct Solution;
