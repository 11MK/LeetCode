pub struct Solution {}

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut i, mut total, mut prev) = (0, 0, i32::MAX);
        while i < prices.len() {
            match prev.cmp(&prices[i]) {
                std::cmp::Ordering::Greater => prev = prices[i],
                std::cmp::Ordering::Less => {
                    total += prices[i] - prev;
                    prev = prices[i]
                }
                _ => (),
            }
            i += 1;
        }
        total
    }
}

fn main() {
    println!("Solution:")
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
