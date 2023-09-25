pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let mut cols: Vec<usize> = Vec::new();
        let mut rows: Vec<usize> = Vec::new();
        for (index, row) in matrix.iter_mut().enumerate() {
            let mut col = 0;
            row.iter_mut().map(|x| {
                if *x == 0 {
                    if !rows.contains(&index) {
                        rows.push(index);
                    }
                    cols.push(col);
                }
                col += 1;
            }).count();
        }
        println!("{:?}", rows);
        while !rows.is_empty() {
            let (mut c,row) = (0, rows.remove(0));
            while c < matrix[0].len() {
                matrix[row][c] = 0;
                println!("TEST");
                c += 1;
            }
        }
        while !cols.is_empty() {
            let (mut r,col) = (0, cols.remove(0));
            while r < matrix.len() {
                matrix[r][col] = 0;
                println!("TEST");
                r += 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
    let result = vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        let result = vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, result);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
        let result = vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, result);
    }
}
