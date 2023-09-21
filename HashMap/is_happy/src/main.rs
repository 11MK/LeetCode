use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let set = HashSet::with_capacity(1000);
        Solution::help(n, set)
    }
    pub fn help(n: i32, mut set: HashSet<i32>) -> bool {
        let mut n = n;
        if set.contains(&n) {
            return false
        } else if n == 1 {
            return true
        }
        set.insert(n);
        let mut sum = 0;
        while n > 0 {
            sum += (n%10) * (n%10);
            n /= 10;
        }
        Solution::help(sum, set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
    }
}
