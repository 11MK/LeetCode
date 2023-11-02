use std::cmp::Ordering;

pub struct Solution {}

const UNITS: &[(i32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I")
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::from("");
        let mut num = num;
        while num > 0 {
            for (unit, value) in UNITS.iter() {
                if num - unit >= 0 {
                    result.push_str(value);
                    num -= unit;
                    break;
                }
            }
        }
        result
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
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
