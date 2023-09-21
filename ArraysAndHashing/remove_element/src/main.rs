pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    while index != nums.len() {
        match nums[index] == val {
            true => {
                nums.remove(index);
            },
            false => index += 1,
        }
        println!("{:?}", nums);
        // println!("{:?}{}", nums[index], val);
    }
    nums.len() as i32
}

fn main() {
    let mut nums: Vec<i32> = vec![3,2,2,3]; 
    assert_eq!(remove_element(&mut nums, 3), 2);
    assert_eq!(nums, vec![2,2]);
}
