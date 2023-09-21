pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut count, mut prev): (i32, i32) = (1, i32::MAX);
    let (mut k, mut index) = (0, 0);
    while index < nums.len() {
        if nums[index] == prev && count >= 2 {
            nums.remove(index);
        }
        else if nums[index] == prev {
            count += 1;
            k += 1;
            index += 1
        }
        else {
            prev = nums[index];
            count = 1;
            k += 1;
            index += 1;
        }
    }
    k
}

fn main() {
    assert_eq!(remove_duplicates(&mut vec![1,1,1,2,2,3]), 5);
    assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,1,2,3,3]), 7);
}
