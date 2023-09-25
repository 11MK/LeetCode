// use std::{collections::HashMap, intrinsics::needs_drop};

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let result: String = String::new();
        let mut missing = t.len();
        let mut need: Vec<u8> = vec![0; 52];
        let (mut i, mut start, mut end) = (0, 0, 0);

        // Iterate through the characters in the input string
        for c in t.chars() {
            need[(c as u8 - b'A') as usize] += 1;
        }

        for (j, ch) in s.chars().skip(1).enumerate() {
            if need[(ch as u8 - b'A') as usize] > 0 {
                missing -= 1;
            }
            need[(ch as u8 - b'A') as usize] -= 1;
            if missing == 0 {
                let c = (s.chars().nth(i).unwrap() as u8 - b'A') as usize;
                while i < j && need[c] < 0 {
                    need[(s.chars().nth(i).unwrap() as u8 - b'A') as usize] += 1;
                    i += 1;
                }
            }
        }
        result
    }
}

fn main() {
    println!("SOLUTION");
    Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }
}
