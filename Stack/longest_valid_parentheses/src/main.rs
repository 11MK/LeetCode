use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        let mut stack: Vec<i32> = Vec::new();
        let mut result: i32 = 0;
        for u in 0..n {
            if s.chars().nth(u).unwrap().eq(&'(') {
                stack.push(u as i32);
            } else {
                if !stack.is_empty() {
                    stack.pop();
                }
                if !stack.is_empty() {
                    result = cmp::max(result, (u - *stack.first().unwrap() as usize) as i32)
                }
                else {
                    stack.push(u as i32)
                }
            }
        }
        result
    }
}

fn main() {
    let s = "()".to_string();
    let res = Solution::longest_valid_parentheses(s);
    println!("{:?}", res)
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0)
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2)
    }
}
