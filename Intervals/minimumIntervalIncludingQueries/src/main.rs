use std::collections::BinaryHeap;

pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    
    return output;
    // for (_index, &n) in queries.iter().enumerate() {
    //     let mut total: i32 = -1;
    //     for (_v, i) in intervals.iter().enumerate() {
    //         let range = i[0]..=i[1];
    //         if range.contains(&n) {
    //             println!("{:?} < {:?} < {:?}", i[0], n, i[1]);
    //             // println!("{:?}, {:?}", x, i[1]);
    //             if total == -1 {
    //                 total = (i[1] - i[0]) + 1;
    //                 continue;
    //             }
    //             if total > (i[1] - i[0] + 1){
    //                 total = (i[1] - i[0]) + 1
    //             }
    //         }
    //     }
        // output.insert(output.len(), total);
    // }
    // println!("{:?}", output);
    // return output;
}

fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,4],vec![2,4],vec![3,6],vec![4,4]];
    let queries: Vec<i32> = vec![2,3,4,5];
    min_interval(intervals, queries);
}
