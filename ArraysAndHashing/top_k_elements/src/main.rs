use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize>  = HashMap::new();
    let numbers = nums.clone();
    let mut result = Vec::new();
    if nums.len() == 1 { return nums; }

    // ------------------ ITERATE THROUGH NUMS -----------------------
    for i in numbers {
        map.entry(i).and_modify(|count| *count += 1).or_insert(1 as usize);
    }
    // -------------------------- SORT ------------------------
    let mut hash_vec: Vec<(&i32, &usize)> = map.iter().collect();
    hash_vec.sort_by(|&a, &b| b.1.cmp(a.1));
    // ------------------- INSERT TO RESULT -----------------------
    while result.len() != k as usize {
        let n = hash_vec.remove(0).clone().0.clone() as i32;
        result.insert(0, n);
    }
    return result;
}

fn main() {
    // let nums = vec![-1, -1];
    let nums = vec![1,1,1,2,2,3];
    let k = 2;
    assert_eq!(vec![1, 2], top_k_frequent(nums, k));
}
