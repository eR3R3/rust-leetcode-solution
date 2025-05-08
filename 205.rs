use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s: HashMap<char, HashSet<i32>> = HashMap::new();
        let mut map_t: HashMap<char, HashSet<i32>> = HashMap::new();
        for (index, char) in s.chars().enumerate() { 
            map_s.entry(char).or_insert(HashSet::new()).insert(index as i32);
        }
        
        for (index, char) in t.chars().enumerate() { 
            map_t.entry(char).or_insert(HashSet::new()).insert(index as i32);
        }
        
        let mut values_s = map_s.values().collect::<Vec<_>>().iter().map(|set| {
            let mut value_s = set.iter().copied().collect::<Vec<_>>();
            value_s.sort();
            value_s
        }).collect::<Vec<_>>();

        let mut values_t = map_t.values().collect::<Vec<_>>().iter().map(|set| {
            let mut value_t = set.iter().copied().collect::<Vec<_>>();
            value_t.sort();
            value_t
        }).collect::<Vec<_>>();

        values_s.sort();
        values_t.sort();

        values_s == values_t
    }
}
