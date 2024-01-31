pub struct Solution {}

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut t1_cost, mut t2_cost) = (std::i32::MAX, std::i32::MAX);
        let (mut t1_profit, mut t2_profit) = (0, 0);
        for x in prices {
            t1_cost = t1_cost.min(x);
            t1_profit = t1_profit.max(x - t1_cost);
            t2_cost = t2_cost.min(x - t1_profit);
            t2_profit = t2_profit.max(x - t2_cost);
        }
        t2_profit
    }
}

fn main() {
    let prices = vec![1, 2, 3, 6, 0, 7, 4, 4];
    println!("{:?}", Solution::max_profit(prices));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
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
