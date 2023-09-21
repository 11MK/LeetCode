pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::{min, max};
    let mut less = vec![];
    let mut more = vec![];
    let mut start = new_interval[0];
    let mut end = new_interval[1];
    
    for curr in intervals{
        if curr[1] < new_interval[0]{
            less.push(curr);
        }
        else if curr[0]>new_interval[1]{
            more.push(curr);
        }
        else {
            start = min(curr[0], start);
            end = max(curr[1], end);
        }
    }
    less.push(vec![start, end]);
    less.append(&mut more);
    less
}

fn main() {
    assert_eq!(insert(vec![vec![1, 3], vec![6, 9]], vec![2,5]), vec![vec![1,5], vec![6, 9]]);
    assert_eq!(insert(vec![vec![1, 2], vec![3, 5], vec![6,7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1,2],vec![3,10],vec![12,16]]);
}
