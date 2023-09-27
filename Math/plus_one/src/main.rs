pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            let sum = digits[i] + carry;
            result[i] = sum % 10;
            carry = sum / 10;
        }
        if carry > 0 {
            result.insert(0, carry);
        }
        result        
    }
}

fn main() {
    println!("Solution:");
    Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::*;

    #[test]
    fn test_1() {
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn test_2() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_3() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 0]);
    }
}
