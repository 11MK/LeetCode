
#[allow(dead_code)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::with_capacity(nums.len());
    nums.into_iter().any(|x| !seen.insert(x))
}

fn main() {
    let array: Vec<i32> = vec![3,1];
    contains_duplicate(array);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let array: Vec<i32> = vec![1,2,3,1];
        assert!(contains_duplicate(array));
    }

    #[test]
    fn test_2() {
        let array: Vec<i32> = vec![1,2,3,4];
        assert!(!contains_duplicate(array));
    }

    #[test]
    fn test_3() {
        let array: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
        assert!(contains_duplicate(array));
    }
}
