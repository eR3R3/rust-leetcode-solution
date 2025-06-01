use std::collections::VecDeque;

impl Solution {
    pub fn is_point_valid(point: (i32, i32), n: usize) -> bool {
        let n = n as i32;
        let x = point.0;
        let y = point.1;
        if x < 0 || y < 0 || x >= n || y >= n {
            return false;
        }
        true
    }

    pub fn dfs(grid: &mut Vec<Vec<i32>>, point: (i32, i32), first_island: &mut Vec<(i32, i32)>, n: usize) {
        // check if the point is still in the boundary or not
        if !Self::is_point_valid(point, n) {
            return;
        }

        // check if the current point is part of the island
        // if yes, push it to the vector for storage
        // if not, end this branch of dfs
        if grid[point.0 as usize][point.1 as usize] == 1 {
            first_island.push(point);
            grid[point.0 as usize][point.1 as usize] = 2;
        } else {
            return;
        }

        // spread to the surrounding points
        for &(dx, dy) in (&[(0, 1), (1, 0), (0, -1), (-1, 0)]).iter() {
            Self::dfs(grid, (point.0 + dx, point.1 + dy), first_island, n);
        }
    }

    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len() as usize;
        let mut first_island: Vec<(i32, i32)> = Vec::new();
        let mut found = false;

        for i in 0..len {
            for j in 0..len {
                if grid[i][j] == 1 {
                    Self::dfs(&mut grid, (i as i32, j as i32), &mut first_island, len);
                    found = true;
                    break;
                }
            }
            if found == true {
                break;
            }
        }
        
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

        for &(x, y) in first_island.iter() {
            queue.push_back((x, y, 0));
        }

        while let Some(point) = queue.pop_front() {
            if !Self::is_point_valid((point.0, point.1), len) {
                continue;
            }

            if visited[point.0 as usize][point.1 as usize] == false {
                // check if the point is already checked before
                visited[point.0 as usize][point.1 as usize] = true;
                // check if the point is already in island 2
                if grid[point.0 as usize][point.1 as usize] == 1 {
                    // the reason we minus one is that the problem is asking the number of time for flipping
                    return point.2 - 1;
                }
            } else {
                continue;
            }

            // add the neighbors to the 
            for &(dx, dy) in (&[(0, 1), (1, 0), (0, -1), (-1, 0)]).iter() {
                let nx = point.0 + dx;
                let ny = point.1 + dy;
                queue.push_back((nx, ny, point.2 + 1));
            }
        }

        -1
    }
}
