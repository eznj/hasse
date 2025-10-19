// lib.rs - Refactored API with vertex label support

use std::collections::HashMap;
use std::hash::Hash;

pub mod gui;
mod graph;

// Re-export the core Graph type
pub use graph::Graph;

/// Builder for creating graphs with a fluent, ergonomic API
/// 
/// # Type Parameters
/// * `V` - The vertex type (defaults to usize for index-based graphs)
/// 
/// # Examples
/// 
/// ```no_run
/// use hasse::graph;
/// 
/// // Simple index-based graph
/// graph()
///     .directed(true)
///     .vertices(5)
///     .edge(0, 1)
///     .edge(1, 2)
///     .build();
/// ```
pub struct GraphBuilder<V = usize> 
where
    V: Eq + Hash + Clone,
{
    num_vertices: usize,
    directed: bool,
    vertex_map: HashMap<V, usize>, // Maps custom vertices to internal indices
    edges: Vec<(usize, usize)>,
    next_index: usize,
}

impl GraphBuilder<usize> {
    /// Create a new graph builder with default settings (undirected, index-based)
    pub fn new() -> Self {
        Self {
            num_vertices: 0,
            directed: false,
            vertex_map: HashMap::new(),
            edges: Vec::new(),
            next_index: 0,
        }
    }

    /// Set the number of vertices for an index-based graph
    /// 
    /// # Example
    /// ```
    /// use hasse::graph;
    /// let g = graph().vertices(10).edge(0, 5).build();
    /// ```
    pub fn vertices(mut self, n: usize) -> Self {
        self.num_vertices = n;
        // Pre-populate vertex map for index-based graphs
        for i in 0..n {
            self.vertex_map.insert(i, i);
        }
        self.next_index = n;
        self
    }
}

impl<V> GraphBuilder<V> 
where
    V: Eq + Hash + Clone + std::fmt::Display,
{
    /// Set whether the graph is directed
    pub fn directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }

    /// Add a single vertex (for custom vertex types)
    /// 
    /// Returns the builder for chaining
    pub fn vertex(mut self, v: V) -> Self {
        if !self.vertex_map.contains_key(&v) {
            self.vertex_map.insert(v, self.next_index);
            self.next_index += 1;
            self.num_vertices += 1;
        }
        self
    }

    /// Add multiple vertices at once
    pub fn vertices_from(mut self, vertices: impl IntoIterator<Item = V>) -> Self {
        for v in vertices {
            if !self.vertex_map.contains_key(&v) {
                self.vertex_map.insert(v, self.next_index);
                self.next_index += 1;
                self.num_vertices += 1;
            }
        }
        self
    }

    /// Add an edge between two vertices
    /// 
    /// Vertices are automatically added if they don't exist yet
    pub fn edge(mut self, from: V, to: V) -> Self {
        // Ensure both vertices exist
        if !self.vertex_map.contains_key(&from) {
            self.vertex_map.insert(from.clone(), self.next_index);
            self.next_index += 1;
            self.num_vertices += 1;
        }
        if !self.vertex_map.contains_key(&to) {
            self.vertex_map.insert(to.clone(), self.next_index);
            self.next_index += 1;
            self.num_vertices += 1;
        }

        let from_idx = *self.vertex_map.get(&from).unwrap();
        let to_idx = *self.vertex_map.get(&to).unwrap();
        self.edges.push((from_idx, to_idx));
        self
    }

    /// Add multiple edges at once
    pub fn edges(mut self, edges: impl IntoIterator<Item = (V, V)>) -> Self {
        for (from, to) in edges {
            self = self.edge(from, to);
        }
        self
    }

    /// Build the final Graph with vertex labels
    pub fn build(self) -> Graph {
        let mut graph = Graph::new(self.num_vertices, self.directed);
        
        // Create vertex labels from the vertex map
        let mut labels = vec![String::new(); self.num_vertices];
        for (vertex, index) in &self.vertex_map {
            labels[*index] = vertex.to_string();
        }
        
        // Set the labels on the graph
        graph.set_labels(labels);
        
        // Add all edges
        for (from, to) in self.edges {
            graph.add_edge(from, to);
        }
        
        graph
    }

    /// Build the graph and immediately display it in the GUI
    /// 
    /// This is a convenience method that combines `build()` and `gui::show()`.
    /// 
    /// # Example
    /// ```no_run
    /// use hasse::graph;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// graph()
    ///     .vertices(3)
    ///     .directed(true)
    ///     .edge(0, 1)
    ///     .edge(0, 2)
    ///     .show()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn show(self) -> Result<(), anyhow::Error> {
        let graph = self.build();
        crate::gui::show(&graph)
    }
}

impl Default for GraphBuilder<usize> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for creating index-based graphs
/// 
/// # Example
/// ```no_run
/// use hasse::graph;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// graph()
///     .vertices(5)
///     .directed(true)
///     .edge(0, 1)
///     .edge(1, 2)
///     .show()?;
/// # Ok(())
/// # }
/// ```
pub fn graph() -> GraphBuilder<usize> {
    GraphBuilder::new()
}

/// Convenience function for creating graphs with custom vertex types
/// 
/// # Example
/// ```no_run
/// use hasse::graph_with;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// graph_with::<&str>()
///     .directed(false)
///     .edge("Alice", "Bob")
///     .edge("Bob", "Charlie")
///     .show()?;
/// # Ok(())
/// # }
/// ```
pub fn graph_with<V>() -> GraphBuilder<V> 
where
    V: Eq + Hash + Clone + std::fmt::Display,
{
    GraphBuilder {
        num_vertices: 0,
        directed: false,
        vertex_map: HashMap::new(),
        edges: Vec::new(),
        next_index: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_directed_graph() {
        let g = graph()
            .directed(true)
            .vertices(3)
            .edge(0, 1)
            .edge(0, 2)
            .edge(1, 2)
            .build();
        
        assert_eq!(g.num_vertices(), 3);
        assert!(g.is_directed());
    }

    #[test]
    fn test_string_vertex_graph() {
        let g = graph_with::<&str>()
            .directed(false)
            .vertex("A")
            .vertex("B")
            .vertex("C")
            .edge("A", "B")
            .edge("B", "C")
            .build();
        
        assert_eq!(g.num_vertices(), 3);
    }

    #[test]
    fn test_convenience_function() {
        let g = graph()
            .vertices(4)
            .directed(true)
            .edges(vec![(0, 1), (1, 2), (2, 3)])
            .build();
        
        assert_eq!(g.num_vertices(), 4);
    }

    #[test]
    fn test_vertex_labels() {
        let g = graph_with::<&str>()
            .edge("Alice", "Bob")
            .edge("Bob", "Charlie")
            .build();
        
        assert_eq!(g.num_vertices(), 3);
        // Labels should be set and accessible via get_label()
    }
}