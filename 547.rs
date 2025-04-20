use std::collections::{HashMap, HashSet};

impl Solution {
  // these are two functions that I used to use but I found other alternatives
    pub fn is_array_all_true(array: &Vec<bool>) -> bool {
        for each in array.iter() {
            if *each == false {
                return false;
            }
        }
        true
    }

    pub fn find_first_false_index(array: &Vec<bool>) -> usize {
        for (index, &each) in array.iter().enumerate() {
            if each == false {
                return index
            }
        }
        0
    }

    // the real code starts from here
    pub fn dfs(graph: &HashMap<i32, HashSet<i32>>, visited: &mut Vec<bool>, current: i32) {
        if visited[current as usize] == true { return; }
        visited[current as usize] = true;
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors.iter() {
                if !visited[neighbor as usize] {
                    Self::dfs(&graph, visited, neighbor.clone());
                }
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        let len = is_connected.len();
        for i in 0 ..= len - 1 {
            graph.insert(i as i32, HashSet::new());
        }

        for i in 0 ..= len - 1 {
            for j in i ..= len - 1 {
                if is_connected[i][j] == 1 {
                    graph.entry(i as i32).and_modify(|set| {set.insert(j as i32);});
                    graph.entry(j as i32).and_modify(|set| {set.insert(i as i32);});
                } else {
                    continue;
                }
            }
        }

        let mut visited: Vec<bool> = vec![false; len];
        let mut cnt: i32 = 0;
        while let Some(start_node) = visited.iter().position(|&x| !x) {
            Self::dfs(&graph, &mut visited, start_node as i32);
            cnt += 1;
        }
        cnt
    }
}
