use std::collections::HashSet;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while !set.contains(&n) {
            set.insert(n);
            let mut sum: i32 = 0;
            for char in n.to_string().chars() {
                let num = char.to_digit(10).unwrap() as i32;
                sum = sum + num * num;
            }
            if sum == 1 {
                return true;
            }
            n = sum;
        }
        false
    }
}
