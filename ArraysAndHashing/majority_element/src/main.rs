pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[nums.len()/2]
}

fn main() {
    assert_eq!(majority_element(vec![3,2,3]), 3);
    assert_eq!(majority_element(vec![2,2,1,1,1,2,2]), 2);
}
