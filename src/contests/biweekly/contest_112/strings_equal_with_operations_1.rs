impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();

        let poss1 = swap(&c1, 0);
        let poss2 = swap(&c1, 1);
        let poss3 = swap(&poss1, 1);

        return
            are_equal(&c1, &c2) ||
            are_equal(&poss1, &c2) ||
            are_equal(&poss2, &c2) ||
            are_equal(&poss3, &c2);
    }
}

fn are_equal(c1: &Vec<char>, c2: &Vec<char>) -> bool {
    for i in [0..c1.len()] {
        if c1[i.clone()] != c2[i] {
            return false;
        }
    }

    return true;
}

fn swap(chars: &Vec<char>, i: usize) -> Vec<char> {
    let j = get_j(i);
    let mut result: Vec<char> = chars.clone();

    let tmp = result[i];
    result[i] = result[j];
    result[j] = tmp;

    return result;
}

fn get_j(i: usize) -> usize {
    return i + 2;
}

pub(crate) struct Solution;