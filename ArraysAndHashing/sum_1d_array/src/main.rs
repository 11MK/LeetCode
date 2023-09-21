pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut previous = 0;
    for (index,n) in nums.iter().enumerate() {
        let sum = previous + n;
        result.insert(index, sum);
        previous = sum;
    }
    result
}

fn main() {
    assert_eq!(running_sum(vec![1,2,3,4]), [1,3,6,10]);
    assert_eq!(running_sum(vec![1,1,1,1,1]), [1,2,3,4,5]);
    assert_eq!(running_sum(vec![3,1,2,10,1]), [3,4,6,16,17]);
}
