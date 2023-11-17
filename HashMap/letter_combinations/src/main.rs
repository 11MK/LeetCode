pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if digits.is_empty() {
            return res;
        }
        let m: std::collections::HashMap<char, &str> = [
            ('1', ""),
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        for c in digits.chars() {
            let letters = m.get(&c).unwrap();
            let mut tmp: Vec<String> = vec![];
            for ch in letters.chars() {
                if res.is_empty() {
                    tmp.push(ch.to_string());
                } else {
                    for r in res.iter() {
                        tmp.push(r.to_owned() + &ch.to_string());
                    }
                }
            }
            res = tmp;
        }
        res
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
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "bd".to_string(),
                "cd".to_string(),
                "ae".to_string(),
                "be".to_string(),
                "ce".to_string(),
                "af".to_string(),
                "bf".to_string(),
                "cf".to_string()
            ]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
