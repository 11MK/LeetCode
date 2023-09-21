pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut running_sum = 0;
    for (i, n) in nums.iter().enumerate() {
        match sum - running_sum - n == running_sum {
            true => return i as i32,
            false => running_sum += n,
        }
    }
    -1
}

fn main() {
    assert_eq!(pivot_index(vec![1,7,3,6,5,6]), 3);
    assert_eq!(pivot_index(vec![1,2,3]), -1);
    assert_eq!(pivot_index(vec![2,-1,1]), 0);
}
