
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r): (usize, usize) = (0, numbers.len()-1);
    use std::cmp::Ordering;
    loop {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Ordering::Less => l += 1,
            Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
            Ordering::Greater => r -= 1,
        }
    }
}


fn main() {
    assert_eq!(two_sum(vec![2,7,11,15], 9), vec![1,2]);
    assert_eq!(two_sum(vec![2,3,4], 6), vec![1,3]);
    assert_eq!(two_sum(vec![-1,0], -1), vec![1,2]);
}
