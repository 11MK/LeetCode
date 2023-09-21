pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);
    let mut max_area = 0;
    use std::cmp;
    while l < r {
        let cur_area = (r - l) as i32 * cmp::min(height[l],height[r]);
        max_area = cmp::max(max_area, cur_area);  
        match height[l] < height[r] {
            true => l += 1,
            false => r -= 1,
        }
    }
    max_area
}

fn main() {
    assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(max_area(vec![1,1]), 1);
}
