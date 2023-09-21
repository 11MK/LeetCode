use std::collections::HashMap;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let result = Vec::new();
    for (index, n) in numbers.iter().enumerate() {
        let diff = target - n;
        match map.get(&diff) {
            Some(i) => {
                if index as i32 == i.clone() as i32 {
                    let res = map.iter().any(|(index, x)| x == i && index.clone() != i.clone() as i32);
                }
                return vec![index as i32, i.clone() as i32];
            },
            None => map.insert(n.clone() as i32, index),
        };
        println!("{:?}, {:?}", n, diff);
    }
    result
}

fn main() {
    let numbers = vec![2,7,11,15];
    let target = 9;
    assert_eq!(vec![1, 2], two_sum(numbers, target));
}
