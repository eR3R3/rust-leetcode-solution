use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s: Vec<&str> = s.split(' ').collect();
        let pattern: Vec<char> = pattern.chars().collect();
        let mut map = HashMap::new();
        let mut used_words = HashMap::new();

        if s.len() != pattern.len() {
            return false;
        }

        for i in 0..s.len() {
            let word = s[i];
            let p = pattern[i];

            // 检查 pattern 字符是否已经映射到某个 word
            if let Some(&mapped_word) = map.get(&p) {
                if mapped_word != word {
                    return false;
                }
            } else {
                // 这个 word 是否已经被别的 pattern 字符用过
                if used_words.contains_key(word) {
                    return false; // 一个 word 被映射给多个 pattern 字符，不行
                }
                map.insert(p, word);
                used_words.insert(word, true);
            }
        }

        true
    }
}
