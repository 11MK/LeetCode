pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut matrix = matrix;
    let mut spiral: Vec<i32> = Vec::new();
    let mut count = 1;
    let mut reverse = true;
    if !matrix.is_empty() {
        spiral.append(&mut matrix.remove(0));
    }
    while !matrix.is_empty() {
        if count == matrix.len() { // REACHED ROW
            let mut row = matrix.remove(0);
            if reverse {
                row.reverse();
                reverse = false;
                count = 1; 
            }
            else {
                reverse = true;
            }
            spiral.append(&mut row);
        }
        else {  // MOVING THROUGH COLUMN
            let mut col = matrix.remove(0);
            let mut val: i32 = 0;
            match reverse {
                true => val = col.remove(col.len()-1),
                false => val = col.remove(0),
            }
            if !col.is_empty() {
                if reverse {
                    matrix.push(col);
                }
                else {
                    matrix.insert(0, col);
                }
            }
            spiral.push(val);
            count += 1;
        }
    }    
    spiral
}
fn main() {
    assert_eq!(spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    assert_eq!(spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16]]), vec![1,2,3,4,8,12,16,15,14,13,9,5,6,7,11,10]);
}


// match count % 2 == 0 {
//     true => {
//         let mut row = matrix.remove(0);
//         if !forward {
//             row.reverse();
//             forward = true
//         }
//         else {
//             forward = false;
//         }
//         spiral.append(&mut row);
//     },
//     false => {
//         let mut col = matrix.remove(0);
//         let mut val: i32 = 0;
//         match !forward {
//             true => val = col.remove(col.len()-1),
//             false => val = col.remove(0),
//         }
//         spiral.push(val);
//         if !col.is_empty() {
//             matrix.push(col);
//         }
//     },
// }
// count += 1;
