#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Some("aba".into()), reorganize("aba"))
    }

    #[test]
    fn example2() {
        assert_eq!(None, reorganize("aaab"))
    }
}

use std::collections::{HashMap, BinaryHeap};
use core::cmp::Ordering;


pub fn reorganize(input: &str) -> Option<String> {
    let mut char_heap = BinaryHeap::<OurValue>::new();
    let mut char_counts = HashMap::<char,usize>::new();
    for c in input.chars() {
        let entry = char_counts.entry(c).or_insert(0);
        *entry += 1;
    }
    for (c, count) in char_counts {
        char_heap.push( OurValue { c, count });
    }

    let mut result = String::new();
    let mut last_char = '\0';
    while char_heap.len() != 0 {
        let mut c1 = char_heap.pop().unwrap();
        if c1.c == last_char {
            match char_heap.pop() {
                Some(mut ov) => {
                    last_char = ov.c;
                    result.push(ov.c);
                    ov.count -= 1;
                    if ov.count > 0 {
                        char_heap.push(ov);
                    }
                },
                None => return None
            }
        } else {
            last_char = c1.c;
            result.push(c1.c);
            c1.count -=1;
        }
        if c1.count > 0 {
            char_heap.push(c1);
        }
    }
    Some(result)
}

struct OurValue {
    c: char,
    count: usize,
}

impl Ord for OurValue {
    fn cmp(&self, other: &OurValue) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for OurValue {
    fn partial_cmp(&self, other: &OurValue) -> Option<Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

impl PartialEq for OurValue {
    fn eq(&self, other: &OurValue) -> bool {
        self.count == other.count
    }
}

impl Eq for OurValue {}

// aaabbc aaaaaaaabbc
// aaaabbbc
// ababababc
