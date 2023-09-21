pub fn trap(height: Vec<i32>) -> i32 {
    let mut water: i32 = 0;
    let mut short_l: (i32, usize) = (i32::MAX, 0);
    let mut tallest_l: (i32, usize) = (i32::MIN, 0);
    let mut index = 0;
    println!("{:?}", height);
    while index < height.len() {
        if height[index] < short_l.0 && height[index] != 0 {
            short_l = (height[index], index);
        }
        if height[index] > tallest_l.0 {
            tallest_l = (height[index], index);
            short_l = (height[index], index);
        }
        println!("{:?}, {:?}", short_l, tallest_l);
        index += 1;
    }
    water
}

fn main() {
    assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(trap(vec![4,2,0,3,2,5]), 6);
}
