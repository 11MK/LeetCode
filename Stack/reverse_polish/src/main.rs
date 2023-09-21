#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut input: Vec<i32> = Vec::new();
    let operators = "+-/*";
    let (mut result, mut index): (i32, usize) = (0, 0);
    if tokens.len() < 2 { return tokens[0].clone().parse::<i32>().unwrap();}
    input.push(tokens[index].clone().parse::<i32>().unwrap());
    index += 1;
    while input.len() != 0 {
        if operators.contains(&tokens[index]) {
            let r = input.pop().unwrap();
            let l = input.pop().unwrap();
            let op = tokens[index].clone();
            match op.as_str() {
                "+" => input.push(l + r),
                "-" => input.push(l - r),
                "*" => input.push(l * r),
                "/" => input.push(l / r),
                _ => continue,
            }
        }
        else {
            let value = tokens[index].clone();
            input.push(value.parse::<i32>().unwrap());
        }
        index += 1;
        // println!("{:?}", input);
        if  input.len() == 1 && index == tokens.len() { result = input.pop().unwrap() }
    }
    result
}

fn main() {
    let mut tokens: Vec<String> = vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()];
    println!("9 = {:?}",  eval_rpn(tokens));

    tokens = vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()];
    println!("6 = {:?}",  eval_rpn(tokens));
}
