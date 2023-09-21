pub fn is_subsequence(s: String, t: String) -> bool {
    let mut subseq_vec: Vec<char> = s.chars().collect();
    if t == *"" && s == *""  {
        return true;
    }
    if t == *"" {
        return false;
    }
    if s == *"" {
        return true;
    }
    for (_,ch) in t.chars().enumerate() {
        if subseq_vec.first().unwrap() == &ch {
            subseq_vec.remove(0);
        }
        if subseq_vec.is_empty() {
            return true;
        }
    }
    false
}

fn main() {
    assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
}
