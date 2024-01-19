#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    match (&arr, k) {
        (_, 0)   => vec![vec![]],
        (&[], _) => Vec::<Vec<i32>>::new(),
        (_, _)   => {
            let mut result = Vec::<Vec<i32>>::new();
            for i in 0..(arr.len() - k + 1) {
            let remaining = combinations(&arr[i + 1..], k - 1);
            for mut combination in remaining {
                let mut current = vec![arr[i]];
                current.append(&mut combination);
                result.push(current);
            }
        }
            result
        },
    }
}
