
pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;
    let mut hm = HashMap::new();

    for c in s.chars() {
         *hm.entry(c).or_insert(0) += 1;
    }

    let mut odd = 0;
    for v in hm.values() {
        if v % 2 == 1 {
            odd += 1;
        }
    }
    if odd > 0 {
        (s.len() - odd + 1) as i32
    } else {
         s.len() as i32
    } 
}

fn main() {
    assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(longest_palindrome("a".to_string()), 1);
}
