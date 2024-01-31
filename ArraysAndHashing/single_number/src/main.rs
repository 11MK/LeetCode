pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        nums.iter().for_each(|x| {
            xor ^= x;
        });
        xor
    }
}

fn main() {
    Solution::single_number(vec![1,1,3,2,4,2,4]);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4)
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::single_number(vec![1]), 1)
    }
}
