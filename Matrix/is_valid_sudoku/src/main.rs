pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
        let mut cols: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
        let mut boxes: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
        for (x, row) in board.iter().enumerate() {
            for (y, ch) in row.iter().enumerate() {
                if !ch.is_ascii_digit() {
                    continue;
                }
                let val = ch.to_digit(10).unwrap() as usize - 1;
                let i = (((3 - (x) % 3) + x) / 3) - 1;
                let j = (((3 - (y) % 3) + y) / 3) - 1;
                let b = match i {
                    0 => j,
                    1 => 3 + j,
                    2 => 6 + j,
                    _ => continue,
                };
                if rows[x][val] == 1 || cols[y][val] == 1 || boxes[b][val] == 1 {
                    return false;
                }
                else {
                    rows[x][val] += 1;
                    cols[y][val] += 1;
                    boxes[b][val] += 1;
                }
            }
        }
        true
    }
}

fn main() {
    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let verify = Solution::is_valid_sudoku(board);
    println!("{:?}", verify)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
