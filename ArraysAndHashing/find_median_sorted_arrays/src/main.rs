pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut n1, mut n2): (Vec<i32>, Vec<i32>) = (nums1, nums2);
        let mut v: Vec<i32> = Vec::new();
        while !n1.is_empty() || !n2.is_empty() {
            if n1.is_empty() {
                v.push(n2.remove(0));
            }
            else if n2.is_empty() {
                v.push(n1.remove(0));
            } else {
                match n1.first().cmp(&n2.first()) {
                    std::cmp::Ordering::Less => v.push(n1.remove(0)),
                    std::cmp::Ordering::Equal => {
                        v.push(n1.remove(0));
                        v.push(n2.remove(0))
                    }
                    std::cmp::Ordering::Greater => v.push(n2.remove(0)),
                }
            }
        }
        match v.len() % 2 == 0 {
            true => {
                let i = v.len() / 2;
                (v[i] + v[i - 1]) as f64 / 2.0
            }
            false => v[v.len() / 2] as f64,            
        }
    }
}

fn main() {
    println!("Solution:");
    Solution::find_median_sorted_arrays(vec![1, 2], vec![3,4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.00000
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.50000
        );
    }
}
