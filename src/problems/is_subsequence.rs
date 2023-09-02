impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let chars: Vec<char> = t.chars().into_iter().collect();
        let mut last_index: usize = 0;

        for c in s.chars().into_iter() {
            let next_index = char_index(&chars[last_index..], c);

            if next_index == -1 {
                return false;
            } else {
                last_index += next_index as usize + 1;
            }
        }

        return true;
    }
}

fn char_index(chars: &[char], c: char) -> isize {
    return match chars.iter().position(|ch| *ch == c) {
        Some(i) => i as isize,
        None => -1,
    }
}

pub(crate) struct Solution;