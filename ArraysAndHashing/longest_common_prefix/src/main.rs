pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.first().unwrap().clone();
        }
        let mut i = 0;
        let s: String = strs[0]
            .clone()
            .chars()
            .take_while(|c| {
                let mut j = 1;
                while j < strs.len() {
                    if strs[j].len() <= i || strs[j].chars().nth(i).unwrap() != *c {
                        return false;
                    }
                    j += 1;
                }
                i += 1;
                true
            })
            .collect();
        s
    }
}

fn main() {
    let strs: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string())
    }

    #[test]
    fn test_2() {
        let strs: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "".to_string())
    }
}
