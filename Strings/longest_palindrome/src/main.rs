use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String { 
        let mut s  = s;
        let (mut low, mut max_len) = (0, 0);
        if s.len() < 2 {
            return s;
        }
        let bytes = s.as_bytes();
        for i in 0..s.len(){
            let (l1, r1) = Solution::expand(bytes, i, i);
            let (l2, r2) = Solution::expand(bytes, i, i+1);
            if (r1 - l1) > max_len {
                low = l1 ;
                max_len = r1 - l1 ;
            }
            if (r2 - l2) > max_len {
                low = l2;
                max_len = r2 - l2;
            }
        }
        String::from_utf8(bytes[low as usize..=max_len as usize].to_vec()).unwrap()
    }

    fn expand(bytes: &[u8], left: usize, right: usize) -> (i32, i32) {
        let (mut l,mut r) = (left, right);
        while r < bytes.len() && bytes[l] == bytes[r] && l != 0 {
            l -= 1;
            r += 1;
        }
        (l-1 as i32, r-1 as i32)
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
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
        assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
    }
}
