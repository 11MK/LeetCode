pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut allocations = vec![1;n];
    use std::cmp;
    for i in 1..n {
        if ratings[i] > ratings[i-1] {
            allocations[i] = allocations[i-1]+1;
        }
    }
    for i in (0..n-1).rev() {
        if ratings[i] > ratings[i+1] {
            allocations[i] = cmp::max(allocations[i], allocations[i+1]+1);
        }
    }
    allocations.into_iter().sum()
}

fn main() {
    assert_eq!(candy(vec![1,0,2]), 5);
    assert_eq!(candy(vec![1,2,2]), 4);
}
