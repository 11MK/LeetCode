pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use Direction::*;
        let mut matrix = matrix;
        let mut spiral: Vec<i32> = vec![];
        let mut dir = Right;
        while !matrix.is_empty() && matrix.iter().any(|v| !v.is_empty()) {
            match dir {
                Down => { matrix.iter_mut().for_each(|v| spiral.push(v.pop().unwrap())) },
                Left => { matrix.pop().unwrap().iter().rev().for_each(|i| spiral.push(*i)) },
                Up => { matrix.iter_mut().rev().for_each(|v| spiral.push(v.remove(0))) },
                Right => spiral.append(&mut matrix.remove(0)),
            };
            dir = next_direction(dir);
        }
        spiral
    }
}

enum Direction {
    Down,
    Left,
    Up,
    Right,
}

fn next_direction(dir: Direction) -> Direction {
    use Direction::*;
    match dir {
        Down => Left,
        Left => Up,
        Up => Right,
        Right => Down,
    }
}

fn main() {
    println!("Solution:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let spiral = Solution::spiral_order(matrix);
        assert_eq!(spiral, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let spiral = Solution::spiral_order(matrix);
        assert_eq!(spiral, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
