// 802. Find Eventual Safe States
// There is a directed graph of n nodes with each node labeled from 0 to n - 1. 
// The graph is represented by a 0-indexed 2D integer array graph where graph[i] is 
// an integer array of nodes adjacent to node i, meaning there is an edge from node i to each node in graph[i].

A node is a terminal node if there are no outgoing edges. A node is a safe node if every possible path starting from that node leads to a terminal node (or another safe node).

Return an array containing all the safe nodes of the graph. The answer should be sorted in ascending order.

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let total_len = graph.len();
        let mut reverse_graph = vec![vec![]; total_len];
        let mut out_degree = vec![0; total_len];
        for (start, ends) in graph.iter().enumerate() {
            out_degree[start] = ends.len();
            for &end in ends.iter() {
                reverse_graph[end as usize].push(start); 
            }
        }

        let mut queue = std::collections::VecDeque::new();
        for (point, &out_num) in out_degree.iter().enumerate() {
            if out_num == 0 {
                queue.push_back(point);
            }
        } 

        let mut safe: Vec<i32> = Vec::new();

        while let Some(point) = queue.pop_front() {
            safe.push(point as i32);
            let ends = &reverse_graph[point as usize];
            for &end in (*ends).iter() {
                out_degree[end as usize] -= 1;
                if out_degree[end] == 0 {
                    queue.push_back(end);
                }
            }
        }

        safe.sort();
        return safe;
    }
}
