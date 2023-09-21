
#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut numbers = nums.clone();
    numbers.sort();
    for (index, v) in numbers.iter().enumerate() {
        if index > 0 && v == numbers.get(numbers.len() - 1).unwrap() {
            continue;
        }
        let (mut l, mut r): (usize, usize) = (index + 1, numbers.len() - 1);
        while l < r {
            let three_sum: i32 = v + numbers[l] + numbers[r];
            if three_sum > 0 {
                r -= 1;
            }
            else if three_sum < 0 {
                l += 1;
            }
            else if result.contains(&vec![*v, numbers[l], numbers[r]]) {
                l += 1;
                continue;
            }
            else {
                result.insert(0, vec![*v, numbers[l], numbers[r]]);
                l += 1;
                while numbers[l] == numbers[l - 1] && l < r {
                    l += 1;
                }
            }
        }
    }
    println!("{:?}", result);
    return result; 
}

fn main() {
    let array: Vec<i32> = vec![-1,0,1,2,-1,-4];
    three_sum(array);
    return;
}
