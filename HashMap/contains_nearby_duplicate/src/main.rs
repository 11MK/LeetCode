pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut distance: HashMap<i32, usize> = HashMap::new();
    for (index, n) in nums.iter().enumerate() {
        match distance.get_mut(n) {
            Some(pos) => {
                if k as usize == (index+1 - *pos) {
                    return true;
                }
                else {
                    distance.insert(*n, index+1);
                }
            },
            None => {
                distance.insert(*n, index+1);
            },
        } 
    }
    false
}

fn main() {
    assert!(contains_nearby_duplicate(vec![1,2,3,1], 3));
    assert!(contains_nearby_duplicate(vec![1,0,1,1], 1));
    assert!(!contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
}
