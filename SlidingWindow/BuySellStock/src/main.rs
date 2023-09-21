#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    let mut low: i32 = -1;
    for p in prices {
        if low == -1 {
            low = p;
            continue;
        }
        if p - low > profit {
            profit = p - low;
            continue;
        }
        if p < low {
            low = p;
        }
    }
    return profit;
}

fn main() {
    let test: Vec<i32> = vec![7,1,5,3,6,4];
    println!("{:?}", max_profit(test));
}

