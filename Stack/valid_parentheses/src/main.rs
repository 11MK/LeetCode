pub fn is_valid(s: String) -> bool {
    let mut list: Vec<char> = s.chars().collect();
    while list.len() != 0 {
        let paren: char = list.pop().unwrap();
        match paren {
            '(' => { if list.contains(&')') { list.remove(list.iter().position(|&r| r == ')').unwrap()); } else {return false;} },
            '[' => { if list.contains(&']') { list.remove(list.iter().position(|&r| r == ']').unwrap()); } else {return false;}},
            '{' => { if list.contains(&'}') { list.remove(list.iter().position(|&r| r == '}').unwrap()); } else {return false;}},
            ')' => { if list.contains(&')') { list.remove(list.iter().position(|&r| r == '(').unwrap()); } else {return false;}},
            ']' => { if list.contains(&']') { list.remove(list.iter().position(|&r| r == '[').unwrap()); } else {return false;}},
            '}' => { if list.contains(&'}') { list.remove(list.iter().position(|&r| r == '{').unwrap()); } else {return false;}},
            _ => { return false; }
        }
    }
    return true;
}

fn main() {
    let s: String = "()".to_string();
    is_valid(s);
}
