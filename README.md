# Hasse

A modern Rust implementation for visualizing Hasse diagrams and working with partially ordered sets (posets).

## What is a Hasse Diagram?

A **Hasse diagram** is a visual way to represent a finite partially ordered set. It shows relationships between elements while removing redundant information.

### Key Concepts

- **Partial Order**: A relation ≤ on a set where:
- **Reflexive**: a ≤ a (every element relates to itself)
- **Antisymmetric**: if a ≤ b and b ≤ a, then a = b
- **Transitive**: if a ≤ b and b ≤ c, then a ≤ c

**Hasse Diagram Properties**:
1. Elements are drawn as nodes
2. If a ≤ b (and a ≠ b), b is placed higher than a
3. Only **direct** relationships are shown with edges
4. Transitive edges are omitted (if a→b and b→c exist, we don't draw a→c)

### Visual Example

Consider the divisibility relation on {1, 2, 3, 6}:

```
Before Reduction:          After Reduction (Hasse Diagram):
    6                              6
   /|\                            / \
  / | \                          /   \
 1  2  3                        2     3
  \ | /                          \   /
   \|/                            \ /
    1                              1
```

The "before" graph shows all divisibility relationships (1 divides everything, 2 and 3 both divide 6, etc.). The Hasse diagram removes redundant edges - we don't draw 1→6 because it's implied by 1→2→6 and 1→3→6.

### How Transitive Reduction Works

This tool implements **transitive reduction** to create Hasse diagrams:

1. For each edge (u, v) in your directed graph
2. Check if there's an alternative path from u to v (not using that edge)
3. If an alternative exists, the edge is redundant - remove it
4. Keep only the minimal edges needed to represent the ordering

**Complexity**: O(V·E) where V is vertices and E is edges.

## Examples

### `graph` usage
```rust
use hasse::graph;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    graph()
        .vertices(5)
        .directed(true)
        .edge(0, 1)
        .edge(1, 2)
        .edge(2, 4)
        .edge(2, 3)
        .show()?;
    Ok(())
}
```

### `graph_with` usage
```rust
use hasse::graph_with;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    graph_with::<&str>()
        .directed(false)
        .edge("Alice", "Bob")
        .edge("Bob", "Charlie")
        .show()?;
    Ok(())
}
```

## Build and Run

### Prereqs

- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs))
- **FLTK dependencies** (for GUI visualization)

### Build

```bash
cargo build
```

### Run

```bash
cargo run --release
```

### Menu Options

After entering your graph, you'll see an interactive menu:

```

=== Hasse Diagram Tool ===
1. Show graph information
2. Compute transitive reduction (Hasse diagram)
3. Compute transitive closure
4. Show topological sort
5. Display graph (GUI)
6. Exit
```

**Option 1: Show graph information**
- Displays vertices, edges, graph type
- Shows source vertices (no incoming edges)
- Shows sink vertices (no outgoing edges)

**Option 2: Compute transitive reduction** ⭐
- Creates the Hasse diagram by removing redundant edges
- Shows before/after edge counts
- Option to replace your graph with the reduction

**Option 3: Compute transitive closure**
- Adds all implied edges (opposite of reduction)
- Useful for understanding full relationships

**Option 4: Show topological sort**
- Displays a valid ordering of vertices
- Only works on directed acyclic graphs (DAGs)

**Option 5: Display graph (GUI)**
- Opens a visual window showing your graph
- Smart layered layout for DAGs
- Circular layout for non-DAGs
- Shows vertex labels and directed edges with arrows

**Option 6: Exit**
- Closes the program

## Common Use Cases

### 1. Subset Inclusion (Power Set)
Visualize how subsets relate under inclusion.

**Example**: Power set of {a, b}
```
Vertices: 0=∅, 1={a}, 2={b}, 3={a,b}
Edges: 0→1, 0→2, 1→3, 2→3 (plus transitive 0→3)
Hasse diagram removes: 0→3
```

### 2. Divisibility Relations
Show which numbers divide others.

**Example**: {1, 2, 3, 6}
```
Vertices: 0=1, 1=2, 2=3, 3=6
All divisibility edges, then reduce to minimal set
```

### 3. Class Hierarchies
Represent inheritance or class relationships.

### 4. Task Dependencies
Show which tasks must complete before others (project management).

## Technical Details

**Algorithms Implemented**:
- Transitive Reduction: O(V·E) using BFS path finding
- Transitive Closure: O(V³) using Floyd-Warshall variant
- Topological Sort: O(V+E) using Kahn's algorithm
- Path Finding: O(V+E) using breadth-first search

## Testing

```bash
cargo test
```

Tests verify:
- Transitive reduction correctness
- Topological sort accuracy
- Path finding algorithms

## Mathematical Background

Hasse diagrams are named after **Helmut Hasse** (1898-1979), though similar diagrams existed earlier. They're fundamental in:

- **Order Theory** - Studying partial orders
- **Lattice Theory** - Visualizing lattice structures  
- **Category Theory** - Representing morphisms
- **Combinatorics** - Analyzing poset properties

The key insight: transitivity is redundant in visual representations. If you can "climb" from A to C via B, you don't need a direct A→C edge.

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.

See the UNLICENSE file.

For more information, please refer to [http://unlicense.org/](http://unlicense.org/)

## Contributing

Improvements welcome! Potential enhancements:
- Export diagrams to SVG/PNG
- Import from Data formats/DOT format  
- Interactive GUI (drag nodes, zoom)
- Additional layout algorithms

