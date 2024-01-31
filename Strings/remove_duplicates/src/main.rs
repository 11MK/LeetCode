pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut i, mut j, mut prev) = (0, 0, None);
        while i < nums.len() {
            if prev != Some(nums[i]) {
                prev = Some(nums[i]);
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
        j as i32
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
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }
}
