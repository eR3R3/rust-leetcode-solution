use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue: VecDeque<char> = VecDeque::new();

        for x in s.chars() {
            if x == '{' || x == '[' || x == '(' {
                queue.push_back(x);  // 入栈
            } else {
                // 从队列尾部出栈并判断是否匹配
                if let Some(last) = queue.pop_back() {
                    if (x == '}' && last != '{') ||
                       (x == ']' && last != '[') ||
                       (x == ')' && last != '(') {
                        return false;  // 如果不匹配则返回 false
                    }
                } else {
                    return false;  // 如果栈为空，则说明多了右括号
                }
            }
        }

        queue.is_empty()  // 如果栈为空，则说明所有括号匹配成功
    }
}
