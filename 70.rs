impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b, mut c) = (0, 0, 1);
        for i in 1..=n {
            a = b;
            b = c;
            c = a + b;
        }
        c
    }
}
