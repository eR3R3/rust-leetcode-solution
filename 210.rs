use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut in_degrees = vec![0; num_courses];

        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

        for prerequisite in prerequisites.into_iter() {
            let inbound = prerequisite[1 as usize];
            let outbound = prerequisite[0 as usize];
            graph.entry(inbound)
                .or_insert(HashSet::new())
                .insert(outbound);
            graph.entry(outbound)
                .or_insert(HashSet::new());
            in_degrees[outbound as usize] += 1;
        }

        let mut queue = VecDeque::new();

        for (index, &in_degree) in in_degrees.iter().enumerate() {
            if in_degree == 0 {
                queue.push_back(index as i32);
            }
        }

        let mut finished = 0; 
        let mut record = Vec::new();

        while let Some(point) = queue.pop_front() {
            finished += 1;
            record.push(point as i32);
            if let Some(set) = graph.get(&point) {
                for &num in set {
                    in_degrees[num as usize] -= 1;
                    if in_degrees[num as usize] == 0 {
                        queue.push_back(num);
                    }
                }
            }
        }

        if finished != num_courses {
            return Vec::new();
        }

        return record;
    }
}
