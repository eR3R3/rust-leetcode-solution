use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        let (start_x, start_y) = (entrance[0] as usize, entrance[1] as usize);

        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y, 0)); // (x, y, steps)

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // 上下左右

        while let Some((x, y, steps)) = queue.pop_front() {
            // 不是入口而且在边界上，就返回步数
            if (x == 0 || x == m - 1 || y == 0 || y == n - 1) && (x != start_x || y != start_y) {
                return steps;
            }

            for (dx, dy) in directions.iter() {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);

                    if maze[new_x][new_y] == '.' {
                        maze[new_x][new_y] = '+'; // 标记为访问过
                        queue.push_back((new_x, new_y, steps + 1));
                    }
                }
            }
        }

        -1 // 没有出口
    }
}
