impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        let len = chars.len();

        for i in 0..len {
            // 奇数长度中心
            let (l1, r1) = Self::expand_around_center(&chars, i, i);
            if r1 - l1 > end - start {
                start = l1;
                end = r1;
            }

            // 偶数长度中心
            if i + 1 < len {
                let (l2, r2) = Self::expand_around_center(&chars, i, i + 1);
                if r2 - l2 > end - start {
                    start = l2;
                    end = r2;
                }
            }
        }

        chars[start..end].iter().collect()
    }

    fn expand_around_center(chars: &[char], mut left: usize, mut right: usize) -> (usize, usize) {
        while left <= right && right < chars.len() && chars[left] == chars[right] {
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }

        // 修正边界：先判断越界再访问内容
        if right >= chars.len() || chars[left] != chars[right] {
            (left + 1, right)
        } else {
            (left, right + 1)
        }
    }
}
