#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.len() == 0 { return String::new(); }
    let first_str = strs[0];
    if strs.len() == 1 { return first_str.to_string(); }

    let mut result = String::new();
    let vec_chars: Vec<_>= strs[1..].into_iter().map(|str| str.chars().collect::<Vec<_>>()).collect();
   
    for (i, char) in first_str.char_indices() {
        for str in &vec_chars {
            if i >= str.len() || char != str[i] {
                return result;
            }
        }
        result.push(char);
    }
    result
}
