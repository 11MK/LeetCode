pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = nums.len() as i32;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] >= target {
                result = mid as i32;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        result
    }
}

fn main() {
    println!("Solution:")
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
