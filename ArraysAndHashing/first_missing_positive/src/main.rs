// pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
//     let mut nums = nums;
//     let len = nums.len();
//     for i in 0..len {
//         while nums[i] > 0 && nums[i] <= len as i32 && nums[i] != nums[nums[i] as usize - 1] {
//             let tmp = nums[i];
//             nums.swap(i, tmp as usize - 1);
//         }
//     }
//     for (i, _) in nums.iter().enumerate().take(len) {
//         if nums[i] != i as i32 + 1 {
//             return i as i32 + 1;
//         }
//     }
//     len as i32 + 1
// }


pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut side_array: Vec<bool> = vec![false; nums.len() + 1];
    for (_,n) in nums.iter().enumerate() {
        if let Some(slot) = side_array.get_mut(*n as usize) {
            *slot = true
        }
    }
    for (index, b) in side_array.iter().enumerate() {
        if !b {
            return index as i32;
        }
    }
    0
}

fn main() {
    assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
