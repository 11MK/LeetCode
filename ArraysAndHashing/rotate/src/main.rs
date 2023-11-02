pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len() as i32;
        let k = if k > l {
            (( k % l )-1) as usize
        } else {
            (k-1) as usize
        };
        let (mut tmp, mut nxt)= (i32::MAX, *nums.first().unwrap());
        let mut moves = 0;
        let mut i = 0;
        while moves != nums.len() {
            let n = if i+k >= l as usize {
                (i + k) % (l-1) as usize
            } else {
                i+k
            };
            tmp = nums[n];
            nums[n] = nxt;
            nxt = tmp;
            i += n;
            moves += 1;
        }
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
        let mut v = &mut vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(v, 3);
        assert_eq!(v, &mut vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        // TODO: - TESTS
        let mut v = &mut vec![-1, -100, 3, 99];
        Solution::rotate(v, 2);
        assert_eq!(v, &mut vec![3, 99, -1, -100]);
    }
}
