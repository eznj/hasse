mod graph;
mod gui;

use anyhow::{Context, Result};
use graph::Graph;
use std::io::{self, BufRead, Write};

fn prompt(msg: &str) -> Result<String> {
    print!("{} ", msg);
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    
    Ok(line.trim().to_string())
}

fn parse_number<T: std::str::FromStr>(prompt_msg: &str) -> Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    loop {
        let input = prompt(prompt_msg)?;
        match input.parse::<T>() {
            Ok(n) => return Ok(n),
            Err(e) => {
                eprintln!("Invalid input: {}. Please try again.", e);
            }
        }
    }
}

fn parse_edge(input: &str) -> Result<(usize, usize)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 2 {
        anyhow::bail!("Expected two numbers separated by space");
    }
    
    let u = parts[0].parse::<usize>()
        .context("Invalid first vertex")?;
    let v = parts[1].parse::<usize>()
        .context("Invalid second vertex")?;
    
    Ok((u, v))
}

fn read_graph_from_input() -> Result<Graph> {
    println!("\n=== Graph Input ===");
    
    let n: usize = parse_number("Enter number of vertices:")?;
    let m: usize = parse_number("Enter number of edges:")?;
    
    let directed_input = prompt("Is graph directed? (Y/N):")?;
    let directed = matches!(directed_input.to_uppercase().as_str(), "Y" | "YES");
    
    let mut graph = Graph::new(n, directed);
    
    println!("\nEnter edges as 'u v' (vertex indices from 0 to {}):", n - 1);
    for i in 0..m {
        loop {
            let input = prompt(&format!("Edge {} of {}:", i + 1, m))?;
            match parse_edge(&input) {
                Ok((u, v)) => {
                    if u >= n || v >= n {
                        eprintln!("Vertices must be in range [0, {})", n);
                        continue;
                    }
                    graph.add_edge(u, v);
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}. Please try again.", e);
                }
            }
        }
    }
    
    Ok(graph)
}

fn print_menu() {
    println!("\n=== Hasse Diagram Tool ===");
    println!("1. Show graph information");
    println!("2. Compute transitive reduction (Hasse diagram)");
    println!("3. Compute transitive closure");
    println!("4. Show topological sort");
    println!("5. Display graph (GUI)");
    println!("6. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn show_graph_info(graph: &Graph) {
    println!("\n=== Graph Information ===");
    println!("Number of vertices: {}", graph.num_vertices());
    println!("Number of edges: {}", graph.num_edges());
    println!("Directed: {}", graph.directed);
    
    graph.print_adjacency_list();
    
    let sources = graph.get_sources();
    let sinks = graph.get_sinks();
    
    if !sources.is_empty() {
        println!("\nSource vertices (no incoming edges): {:?}", sources);
    }
    if !sinks.is_empty() {
        println!("Sink vertices (no outgoing edges): {:?}", sinks);
    }
}

fn main() -> Result<()> {
    println!("Welcome to Hasse Diagram Visualization Tool!");
    println!("============================================\n");
    
    let mut graph = read_graph_from_input()?;
    
    loop {
        print_menu();
        
        let choice = prompt("")?;
        
        match choice.trim() {
            "1" => {
                show_graph_info(&graph);
            }
            "2" => {
                if !graph.directed {
                    eprintln!("Error: Transitive reduction only works on directed graphs.");
                    continue;
                }
                
                println!("\nComputing transitive reduction...");
                let reduced = graph.transitive_reduction();
                
                println!("\n=== Hasse Diagram (Transitive Reduction) ===");
                println!("Original edges: {}", graph.num_edges());
                println!("Reduced edges: {}", reduced.num_edges());
                reduced.print_adjacency_list();
                
                let replace = prompt("\nReplace current graph with reduction? (Y/N):")?;
                if matches!(replace.to_uppercase().as_str(), "Y" | "YES") {
                    graph = reduced;
                    println!("Graph replaced with transitive reduction.");
                }
            }
            "3" => {
                println!("\nComputing transitive closure...");
                let closure = graph.transitive_closure();
                
                println!("\n=== Transitive Closure ===");
                println!("Original edges: {}", graph.num_edges());
                println!("Closure edges: {}", closure.num_edges());
                closure.print_adjacency_list();
                
                let replace = prompt("\nReplace current graph with closure? (Y/N):")?;
                if matches!(replace.to_uppercase().as_str(), "Y" | "YES") {
                    graph = closure;
                    println!("Graph replaced with transitive closure.");
                }
            }
            "4" => {
                if !graph.directed {
                    eprintln!("Error: Topological sort only works on directed graphs.");
                    continue;
                }
                
                match graph.topological_sort() {
                    Some(order) => {
                        println!("\n=== Topological Sort ===");
                        println!("Order: {:?}", order);
                    }
                    None => {
                        eprintln!("Error: Graph contains a cycle, cannot perform topological sort.");
                    }
                }
            }
            "5" => {
                println!("\nOpening GUI window...");
                gui::show(&graph)?;
            }
            "6" => {
                println!("\nGoodbye!");
                break;
            }
            _ => {
                eprintln!("Invalid choice. Please select 1-6.");
            }
        }
    }
    
    Ok(())
}
