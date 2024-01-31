pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        *nums = nums.iter().rev().copied().collect();
        let (left ,right) = nums.split_at_mut(k as usize);
        left.reverse();
        right.reverse();
    }
}

fn main() {
    Solution::rotate(&mut vec![1,2,3,4,5,6], 8);
    Solution::rotate(&mut vec![1,2,3,4,5,6], 4);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        // TODO: - TESTS
        let v = &mut vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(v, 3);
        assert_eq!(v, &mut vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        // TODO: - TESTS
        let v: &mut Vec<i32> = &mut vec![-1, -100, 3, 99];
        Solution::rotate(v, 2);
        assert_eq!(v, &mut vec![3, 99, -1, -100]);
    }
}
