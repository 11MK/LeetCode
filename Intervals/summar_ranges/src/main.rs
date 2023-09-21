
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let (mut lower, mut upper): (i32, i32) = (0, 0);
    let mut n = 0;
    while n < nums.len() {
        lower = nums[n];
        upper = nums[n];
        while n+1 < nums.len() && nums[n+1] == upper + 1  {
            n += 1;
            upper += 1;
        }
        if lower == upper {
            result.push(format_args!("{}", lower).to_string());
        }
        else {
            result.push(format_args!("{}->{}", lower, upper).to_string());
        }
        n += 1;
    }
    result
}

fn main() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]
    );
    assert_eq!(
        summary_ranges(vec![0,2,3,4,6,8,9]),
        vec!["0".to_string(), "2->4".to_string(), "6".to_string(), "8->9".to_string()]
    )
}
