use std::collections::HashMap;

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    let mut sum_indices: Vec<i32> = vec![];
    for (index, &n) in nums.iter().enumerate() {
        let dif = target - n;
        if let Some(&i) = index_hashmap.get(&dif) {
            sum_indices =  vec![i as i32, index as i32];
        } else {
            index_hashmap.insert(n, index as i32);
        }
    }
    sum_indices
}

fn main() {
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let array: Vec<i32> = vec![2,7,11,15];
        let target = 9;
        assert_eq!(two_sum(array, target), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let array: Vec<i32> = vec![3,2,4];
        let target = 6;
        assert_eq!(two_sum(array, target), vec![1,2]);
    }

    #[test]
    fn test_3() {
        let array: Vec<i32> = vec![3,3];
        let target = 6;
        assert_eq!(two_sum(array, target), vec![0,1]);
    }
}
