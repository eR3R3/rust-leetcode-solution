impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();
        let shortest_len = shortest.len();
        let mut round = 0;

        for i in 0..shortest_len {
            let ch = strs[0].chars().nth(i).unwrap();
            if strs.iter().all(|s| s.chars().nth(i).unwrap() == ch) {
                round += 1;
            } else {
                break;
            }
        }

        strs[0].chars().take(round).collect()
    }
}
