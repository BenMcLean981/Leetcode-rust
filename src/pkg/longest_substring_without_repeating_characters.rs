use std::{cmp, collections::HashSet};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().into_iter().collect();

        let mut crawler = Crawler::new();
        let mut longest: usize = 0;

        loop {
            longest = cmp::max(longest, crawler.len());

            if crawler.is_done(&chars) {
                break;
            } else if crawler.can_grow(&chars) {
                crawler.grow(&chars);
            } else {
                crawler.shrink(&chars);
            }
        }

        return longest as i32;
    }
}

struct Crawler {
    start: usize,
    end: usize,
    chars: HashSet<char>,
}

impl Crawler {
    pub fn new() -> Crawler {
        return Crawler {
            start: 0,
            end: 0,
            chars: HashSet::<char>::new()
        };
    }

    pub fn len(&self) -> usize {
        return self.end - self.start;
    }

    pub fn can_grow(&self, chars: &Vec<char>) -> bool {
        let contains_next_char = self.chars.contains(&self.get_next_char(chars));
        
        return !self.is_done(chars) && !contains_next_char;
    }

    pub fn is_done(&self, chars: &Vec<char>) -> bool {
        return self.end == chars.len();
    }

    pub fn grow(&mut self, chars: &Vec<char>) {
        self.end += 1;
        self.chars.insert(self.get_last_char(chars));
    }

    pub fn shrink(&mut self, chars: &Vec<char>) {
        self.chars.remove(&self.get_first_char(chars));
        self.start += 1;
    }

    fn get_first_char(&self, chars: &Vec<char>) -> char {
        return self.get_char(chars, self.start);
    }

    fn get_last_char(&self, chars: &Vec<char>) -> char {
        return self.get_char(chars, self.end - 1);
    }

    fn get_next_char(&self, chars: &Vec<char>) -> char {
        return self.get_char(chars, self.end);
    }

    fn get_char(&self, chars: &Vec<char>, idx: usize) -> char {
        return chars[idx];
    }
}

pub (crate) struct Solution;
