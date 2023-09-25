pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (rows, cols): (usize, usize) = (matrix.len() - 1, matrix[0].len() - 1);
        let (mut r, mut c): (usize, usize) = (0, 0);

        let mut d = 1;
        let mut tmp = matrix[0][0];
        let mut moves = 0;
        while moves < (matrix.len() * matrix[0].len()) {
            if moves % 4 == 0 && moves != 0 {
                if c == cols - d {
                    r = d;
                    c = d;
                    d += 1;
                } else {
                    c += 1;
                }
                tmp = matrix[r][c];
            }
            let (x, y) = (c, (rows as i32 - r as i32) as usize);
            r = x;
            c = y;
            std::mem::swap(&mut tmp, &mut matrix[x][y]);
            moves += 1;
        }
    }
}

fn main() {
    println!("Solution:");
    // let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // let result: Vec<Vec<i32>> = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

    // let mut matrix: Vec<Vec<i32>> = vec![
    //     vec![5, 1, 9, 11],
    //     vec![2, 4, 8, 10],
    //     vec![13, 3, 6, 7],
    //     vec![15, 14, 12, 16],
    // ];
    // let result: Vec<Vec<i32>> = vec![
    //     vec![15, 13, 2, 5],
    //     vec![14, 3, 4, 1],
    //     vec![12, 6, 8, 9],
    //     vec![16, 7, 10, 11],
    // ];

    let mut matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];
    let result: Vec<Vec<i32>> = vec![
        vec![21, 16, 11, 6, 1],
        vec![22, 17, 12, 7, 2],
        vec![23, 18, 13, 8, 3],
        vec![24, 19, 14, 9, 4],
        vec![25, 20, 15, 10, 5],
    ];
    Solution::rotate(&mut matrix);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let result: Vec<Vec<i32>> = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, result);
    }

    #[test]
    fn test_2() {
        let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result: Vec<Vec<i32>> = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, result);
    }
}
