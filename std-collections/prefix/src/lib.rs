#![forbid(unsafe_code)]

pub fn longest_common_prefix(mut strs: Vec<&str>) -> String {
    if strs.len() == 0 { return String::new(); }

    let last_string_chars = strs.pop().unwrap().chars();
    let mut result: Vec<char> = vec![];

    let char_iters: Vec<_>= strs.into_iter().map(|str| str.chars()).collect();
    
    // for in_char in last_string_chars {
    //     for char_iter in &char_iters {
    //         let out_char: Option<char> = char_iter.next();
    //         if out_char.is_none() || in_char != out_char.unwrap() { return result.into_iter().collect(); }
    //         let out_char = out_char.unwrap();
    //     }
    //     result.push(in_char);
    // }

    result.into_iter().collect()
}
