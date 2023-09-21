
#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 
    {
        let mut max_len: usize = 0;
        // [1] longest substring is the one with the largest
        //     difference of positions of repeated characters;
        //     thus, we should create a storage for such positions
        let mut pos: [usize;128] = [0;128];
        // [2] while iterating through the string (i.e., moving
        //     the end of the sliding window), we should also
        //     update the start of the window
        let mut start: usize = 0;
        for (end, ch) in s.chars().enumerate()
        {
            // [3] get the position for the start of sliding window
            //     with no other occurences of 'ch' in it
            start = start.max(pos[ch as usize]);
            // [4] update maximum length 
            max_len = max_len.max(end-start+1);
            // [5] set the position to be used in [3] on next iterations
            pos[ch as usize] = end + 1;
        }
        return max_len as i32;
    }
// pub fn length_of_longest_substring(s: String) -> i32 {
//     let mut length: i32 = 0;  
//     let char_vec: Vec<char> = s.chars().collect();
//     let (mut l, mut r): (usize, usize) = (0, 0);
//     while r != char_vec.len() {
//         let mut tmp: Vec<char> = vec![];
//         let mut indexes: HashMap<char, usize> = HashMap::new();
//         tmp.insert(0, char_vec[l]);
//         indexes.insert(char_vec[l], l);
//         r = l + 1;
//         while r != char_vec.len() {
//             if tmp.contains(&char_vec[r]) {    
//                 if l == indexes.get(&char_vec[r]).unwrap().clone() {
//                     l += 1;
//                     break;
//                 } 
//                 l = indexes.get(&char_vec[r]).unwrap().clone();
//                 break;
//             }
//             else {
//                 tmp.insert(0, char_vec[r]);
//                 indexes.insert(char_vec[r], r);
//                 r += 1;
//             }
//         }
//         if tmp.len() as i32 > length {
//             length = tmp.len() as i32;
//         }
//     }
//     return length;
// }

fn main() {
    let string = "bbbbb".to_string();
    length_of_longest_substring(string);
    return;
}
