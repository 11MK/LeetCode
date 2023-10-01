pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let forward = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        let reverse = forward.chars().rev().collect::<String>();
        forward == reverse
    }
}

fn main() {
    println!("Solution:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ),);
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }
}
