#[allow(dead_code)]
fn is_anagram(s: String, t: String) -> bool {
    let mut a = [0; 26];
    let mut b = [0; 26];
    for c in s.chars() {
        a[c as usize - 'a' as usize] += 1;
    }
    for c in t.chars() {
        b[c as usize - 'a' as usize] += 1;
    }
    return a == b;
    // let s_string: &str = &s[..];
    // let mut chars: Vec<char> = s_string.chars().collect();
    // chars.sort_by(|a, b| b.cmp(a));
    // let forward = String::from_iter(chars);
    //
    // let t_string: &str = &t[..];
    // let mut chars: Vec<char> = t_string.chars().collect();
    // chars.sort_by(|a, b| b.cmp(a));
    // let reverse = String::from_iter(chars);
    //
    // return forward == reverse;
}

fn main() {
    is_anagram("anagram".to_string(), "nagaram".to_string());
    is_anagram("rat".to_string(), "car".to_string());
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let forward: String = "anagram".to_string(); 
        let reverse: String = "nagaram".to_string(); 
        assert_eq!(is_anagram(forward, reverse), true);
    }

    #[test]
    fn test_not_anagram() {
        let forward: String = "rat".to_string(); 
        let reverse: String = "car".to_string(); 
        assert_eq!(is_anagram(forward, reverse), false);
    }
}
