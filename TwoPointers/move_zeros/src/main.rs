pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }
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
        let mut v = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1,3,12,0,0])
    }
    #[test]
    fn test_2() {
        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![0])
    }

    #[test]
    fn test_3() {
        let mut v = vec![1,0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1,0])
    }
}
