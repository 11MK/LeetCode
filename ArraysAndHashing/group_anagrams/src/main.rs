
#[allow(dead_code)]
fn group_anagram(s: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let result: Vec<Vec<String>> = Vec::new();
    let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
    for c in s {
        let mut a = [0; 26];
        for ch in c.chars() {
            a[ch as usize - 'a' as usize] += 1;
        }
        match map.contains_key(&a) {
            true => map.get(&a).unwrap().insert(0, c.clone()),
            false => todo!(),
        }      
    }
    return result;
}

fn main() {
    print!("Test")
}   

#[cfg(test)]
mod tests {
    use crate::group_anagram;

    #[test]
    fn test_1() {
        let inp: Vec<String> = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
        let result: Vec<Vec<String>> = group_anagram(inp);
        print!("{:?}", result);
    }
    //
    // #[test]
    // fn test_2() {
    //     [""]
    //     let forward: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"]; 
    //     assert_eq!(is_anagram(forward, reverse), false);
    // }
    //
    // #[test]
    // fn test_3() {
    //     let forward: String = "rat".to_string(); 
    //     let reverse: String = "car".to_string(); 
    //     assert_eq!(is_anagram(forward, reverse), false);
    // }
}
