use std::collections::HashSet;

#[allow(dead_code)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(nums.len());
    nums.into_iter().any(|x| !seen.insert(x))
    // let mut values: Vec<i32> = nums.clone();
    // let mut i: usize = 0;
    // while i != nums.len() {
    //     let x = values.pop().unwrap();
    //     match nums.contains(&nums[x as usize]) {
    //         true => return true,
    //         false => i += 1,
    //     }
    // }
    // return false;
}

fn main() {
    let array: Vec<i32> = vec![3,1];
    contains_duplicate(array);
    return;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let array: Vec<i32> = vec![1,2,3,1];
        assert_eq!(contains_duplicate(array), true);
    }

    #[test]
    fn test_2() {
        let array: Vec<i32> = vec![1,2,3,4];
        assert_eq!(contains_duplicate(array), false);
    }

    #[test]
    fn test_3() {
        let array: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
        assert_eq!(contains_duplicate(array), true);
    }
}
