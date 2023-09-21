fn reverse_words(s: String) -> String {
    let mut tmp: String = "".to_string();
    let mut words: Vec<String> = Vec::new();
    let st = s.chars();
    for (index, ch) in st.into_iter().enumerate() {
        if let ' ' = ch {
            if !tmp.is_empty() {
                words.insert(0, tmp);
                tmp = "".to_string();
            }
        } else if index == s.len() - 1 {
            tmp.push(ch);
            words.insert(0, tmp.clone());
        } else {
            tmp.push(ch);
        }
    }
    words.join(" ")
}

fn reverse_words_best(s: String) -> String {
    let reversed: Vec<&str> = s.split_whitespace().rev().collect();
    reversed.join(" ")
}

fn main() {
    assert_eq!(
        reverse_words("the sky is blue".to_string()),
        "blue is sky the"
    );
    assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
    assert_eq!(
        reverse_words("a good   example".to_string()),
        "example good a"
    );

    assert_eq!(
        reverse_words_best("the sky is blue".to_string()),
        "blue is sky the"
    );
    assert_eq!(reverse_words_best("  hello world  ".to_string()), "world hello");
    assert_eq!(
        reverse_words_best("a good   example".to_string()),
        "example good a"
    );
}
