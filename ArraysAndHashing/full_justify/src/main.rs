pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut strs = words;
    let mut tmp: Vec<String> = Vec::new();
    let maxwidth: usize = max_width as usize;
    let (mut str_len, mut spaces) = (0, 0);
    while !strs.is_empty() {
        spaces = 0;
        if tmp.is_empty() {
            str_len = 0;
        }
        else {
            spaces = tmp.len() - 1;
        }
        let word = strs.remove(0);
        // println!("{:?}", word.len() + str_len + spaces);
        match (word.len() + str_len + spaces).cmp(&(maxwidth)) {
            std::cmp::Ordering::Less => {
                str_len += word.len();
                tmp.push(word);
                if strs.is_empty() {
                    let padding = " ".repeat(maxwidth - tmp[0].len());
                    tmp[0].push_str(&padding);
                    result.push(tmp[0].clone());
                }
            },
            std::cmp::Ordering::Equal => {
                result.push(tmp.join(" "));
                tmp.clear();
            },
            std::cmp::Ordering::Greater => {
                strs.insert(0, word);
                if spaces == 0 {
                    let padding = " ".repeat(maxwidth - tmp[0].len());
                    tmp[0].push_str(&padding);
                    result.push(tmp[0].clone());
                }
                else {
                    let padding = (maxwidth - str_len) / spaces;
                    if (padding * spaces) + str_len < maxwidth {
                        tmp[0].push_str(&' '.to_string());
                    }
                    let filler = " ".repeat(padding);
                    result.push(tmp.join(&filler));
                }
                tmp.clear()
            },
        }
    }
    result
}

fn main() {
    assert_eq!(
        full_justify(
            vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string()
            ],
            16
        ),
        vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string()
        ]
    );

    assert_eq!(
        full_justify(
            vec![
                "Science".to_string(),
                "is".to_string(),
                "what".to_string(),
                "we".to_string(),
                "understand".to_string(),
                "well".to_string(),
                "enough".to_string(),
                "to".to_string(),
                "explain".to_string(),
                "to".to_string(),
                "a".to_string(),
                "computer.".to_string(),
                "Art".to_string(),
                "is".to_string(),
                "everything".to_string(),
                "else".to_string(),
                "we".to_string(),
                "do".to_string()
            ],
            20
        ),
        vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string()
        ]
    )
}


// pub fn full_justify2(words: Vec<String>, max_width: i32) -> Vec<String> {
//     let mut result: Vec<String> = Vec::new();
//     let mut strs = words;
//     let mut tmp: Vec<String> = Vec::new();
//     let mut index: usize = 0;
//     let mut str_len = 0;
//     let mut min_spaces = 0;
//     while index != strs.len() {
//         if !tmp.is_empty() {
//             min_spaces = tmp.len() - 1;
//         }
//         else {
//             min_spaces = 0;
//         }
//         println!("{:?} {:?}", strs[index], strs[index].len());
//         match (strs[index].len() + str_len + min_spaces).cmp(&(max_width as usize)) {
//             std::cmp::Ordering::Less => {
//                 tmp.push(strs[index].to_string());
//                 str_len += strs[index].len();
//                 index += 1;
//             }
//             std::cmp::Ordering::Equal => {
//                 tmp.push(strs[index].to_string());
//                 result.push(tmp.join(" "));
//                 tmp.clear();
//                 str_len = 0;
//                 index += 1;
//             }
//             std::cmp::Ordering::Greater => {
//                 let mut padding = " ".to_string();
//                 if min_spaces > 0 {
//                     let max_spaces = (max_width as usize - str_len) / min_spaces;
//                     padding = " ".repeat(max_spaces);
//                     result.push(tmp.join(&padding));
//                 }
//                 else {
//                     let max_spaces = max_width as usize - str_len;
//                     padding = " ".repeat(max_spaces);
//                     tmp[0].push_str(&padding);
//                     result.push(tmp[0].clone());
//                 }
//                 str_len = 0;
//                 tmp.clear();
//             }
//         }
//     }
//     result
// }
