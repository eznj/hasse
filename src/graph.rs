use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Graph {
    pub adj: Vec<Vec<usize>>,
    pub directed: bool,
    vertex_labels: Vec<String>,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            directed,
            vertex_labels: (0..n).map(|i| i.to_string()).collect(),
        }
    }
    
    #[allow(dead_code)]
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    #[allow(dead_code)]
    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.vertex_labels = labels;
    }

    /// Get the label for a vertex (used by GUI)
    pub fn get_label(&self, index: usize) -> &str {
        self.vertex_labels
            .get(index)
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        if !self.directed {
            self.adj[v].push(u);
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    pub fn num_edges(&self) -> usize {
        if self.directed {
            self.adj.iter().map(|edges| edges.len()).sum()
        } else {
            self.adj.iter().map(|edges| edges.len()).sum::<usize>() / 2
        }
    }

    /// Check if there's a path from u to v (used for transitive reduction)
    pub fn has_path(&self, u: usize, v: usize) -> bool {
        if u == v {
            return true;
        }

        let mut visited = vec![false; self.adj.len()];
        let mut queue = VecDeque::new();
        queue.push_back(u);
        visited[u] = true;

        while let Some(curr) = queue.pop_front() {
            for &next in &self.adj[curr] {
                if next == v {
                    return true;
                }
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }

        false
    }

    /// Find shortest path length from src to target, avoiding a specific edge
    pub fn shortest_path_avoiding(&self, src: usize, target: usize, avoid: (usize, usize)) -> Option<usize> {
        if src == target {
            return Some(0);
        }

        let mut visited = vec![false; self.adj.len()];
        let mut queue = VecDeque::new();
        queue.push_back((src, 0));
        visited[src] = true;

        while let Some((curr, dist)) = queue.pop_front() {
            for &next in &self.adj[curr] {
                // Skip the edge we want to avoid
                if (curr, next) == avoid {
                    continue;
                }

                if next == target {
                    return Some(dist + 1);
                }

                if !visited[next] {
                    visited[next] = true;
                    queue.push_back((next, dist + 1));
                }
            }
        }

        None
    }

    /// Transitive reduction - removes edges that can be inferred by transitivity
    /// Essential for creating Hasse diagrams from partial orders
    pub fn transitive_reduction(&self) -> Graph {
        if !self.directed {
            panic!("Transitive reduction only works on directed graphs");
        }

        let mut reduced = Graph::new(self.num_vertices(), true);

        // For each edge (u, v), keep it only if there's no path from u to v through other vertices
        for u in 0..self.adj.len() {
            for &v in &self.adj[u] {
                // Check if there's an alternative path from u to v that doesn't use this edge
                if self.shortest_path_avoiding(u, v, (u, v)).is_none() {
                    // No alternative path exists, so this edge is necessary
                    reduced.add_edge(u, v);
                }
            }
        }

        reduced
    }

    /// Transitive closure - adds all edges implied by transitivity
    pub fn transitive_closure(&self) -> Graph {
        let n = self.num_vertices();
        let mut closure = self.clone();

        // Floyd-Warshall-like algorithm for transitive closure
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if closure.has_path(i, k) && closure.has_path(k, j) {
                        if !closure.adj[i].contains(&j) {
                            closure.adj[i].push(j);
                        }
                    }
                }
            }
        }

        closure
    }

    /// Get vertices with no incoming edges (sources in a DAG)
    pub fn get_sources(&self) -> Vec<usize> {
        let mut has_incoming = vec![false; self.num_vertices()];
        
        for edges in &self.adj {
            for &v in edges {
                has_incoming[v] = true;
            }
        }

        (0..self.num_vertices())
            .filter(|&i| !has_incoming[i])
            .collect()
    }

    /// Get vertices with no outgoing edges (sinks in a DAG)
    pub fn get_sinks(&self) -> Vec<usize> {
        (0..self.num_vertices())
            .filter(|&i| self.adj[i].is_empty())
            .collect()
    }

    /// Topological sort (useful for Hasse diagram layout)
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        if !self.directed {
            return None;
        }

        let mut in_degree = vec![0; self.num_vertices()];
        for edges in &self.adj {
            for &v in edges {
                in_degree[v] += 1;
            }
        }

        let mut queue: VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|&(_, deg)| *deg == 0)
            .map(|(i, _)| i)
            .collect();

        let mut result = Vec::new();

        while let Some(u) = queue.pop_front() {
            result.push(u);

            for &v in &self.adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if result.len() == self.num_vertices() {
            Some(result)
        } else {
            None // Graph has a cycle
        }
    }

    pub fn print_adjacency_list(&self) {
        println!("\nAdjacency List:");
        for (i, edges) in self.adj.iter().enumerate() {
            if !edges.is_empty() {
                print!("Vertex {}: ", i);
                for (j, &v) in edges.iter().enumerate() {
                    print!("{}", v);
                    if j < edges.len() - 1 {
                        print!(", ");
                    }
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transitive_reduction() {
        // Create a graph: 0 -> 1 -> 2, with 0 -> 2 (transitive edge)
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2); // This should be removed

        let reduced = g.transitive_reduction();
        
        // Should only have edges 0->1 and 1->2
        assert_eq!(reduced.num_edges(), 2);
        assert!(reduced.adj[0].contains(&1));
        assert!(reduced.adj[1].contains(&2));
        assert!(!reduced.adj[0].contains(&2));
    }

    #[test]
    fn test_topological_sort() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);

        let topo = g.topological_sort();
        assert!(topo.is_some());
        
        let order = topo.unwrap();
        assert_eq!(order.len(), 4);
        assert_eq!(order[0], 0); // 0 should be first
        assert_eq!(order[3], 3); // 3 should be last
    }
}

