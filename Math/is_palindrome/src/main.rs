pub fn is_palindrome(x: i32) -> bool {
    let forward: Vec<char> = x.to_string().chars().collect();
    let mut reverse = forward.clone();
    reverse.reverse();
    l
    forward == reverse
}

fn main() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(-10));
}
