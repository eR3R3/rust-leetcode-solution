impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut vec_s = s.chars().collect::<Vec<_>>();
        let mut vec_t = t.chars().collect::<Vec<_>>();
        vec_s.sort();
        vec_t.sort();
        vec_t == vec_s
    }
}
