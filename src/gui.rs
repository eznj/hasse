use crate::graph::Graph;
use anyhow::Result;
use fltk::{app, draw, enums::Color, frame::Frame, prelude::*, window::Window};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const NODE_RADIUS: i32 = 40;
const MARGIN: i32 = 60;

/// Calculate positions for vertices in a layered layout (good for DAGs/Hasse diagrams)
fn calculate_layered_positions(graph: &Graph) -> Vec<(f64, f64)> {
    let n = graph.num_vertices();
    
    if n == 0 {
        return Vec::new();
    }
    
    // Try topological sort to determine layers
    let mut layers: Vec<Vec<usize>> = Vec::new();
    
    if let Some(topo_order) = graph.topological_sort() {
        // Assign layers based on longest path from sources
        let mut layer_map = vec![0; n];
        
        for &v in &topo_order {
            let mut max_layer = 0;
            
            // Find all vertices that point to v
            for u in 0..n {
                if graph.adj[u].contains(&v) {
                    max_layer = max_layer.max(layer_map[u] + 1);
                }
            }
            
            layer_map[v] = max_layer;
        }
        
        // Group vertices by layer
        let max_layer = *layer_map.iter().max().unwrap_or(&0);
        layers.resize(max_layer + 1, Vec::new());
        
        for (v, &layer) in layer_map.iter().enumerate() {
            layers[layer].push(v);
        }
    } else {
        // Fallback: circular layout
        return calculate_circular_positions(n);
    }
    
    // Calculate positions based on layers
    let mut positions = vec![(0.0, 0.0); n];
    let height = (WINDOW_HEIGHT - 2 * MARGIN) as f64;
    let width = (WINDOW_WIDTH - 2 * MARGIN) as f64;
    
    let num_layers = layers.len();
    
    for (layer_idx, layer) in layers.iter().enumerate() {
        let y = MARGIN as f64 + (layer_idx as f64 / (num_layers - 1).max(1) as f64) * height;
        let layer_width = width / (layer.len() + 1) as f64;
        
        for (pos_in_layer, &v) in layer.iter().enumerate() {
            let x = MARGIN as f64 + (pos_in_layer + 1) as f64 * layer_width;
            positions[v] = (x, y);
        }
    }
    
    positions
}

/// Calculate positions for vertices in a circular layout
fn calculate_circular_positions(n: usize) -> Vec<(f64, f64)> {
    let center_x = WINDOW_WIDTH as f64 / 2.0;
    let center_y = WINDOW_HEIGHT as f64 / 2.0;
    let radius = ((WINDOW_WIDTH.min(WINDOW_HEIGHT) / 2 - MARGIN) as f64) * 0.8;
    
    (0..n)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::PI / 2.0;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            (x, y)
        })
        .collect()
}

/// Draw the graph
fn draw_graph(graph: &Graph, positions: &[(f64, f64)]) {
    // Clear background
    draw::set_draw_color(Color::White);
    draw::draw_rectf(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    
    // Draw edges first (so they appear behind nodes)
    draw::set_draw_color(Color::from_rgb(100, 100, 100));
    draw::set_line_style(draw::LineStyle::Solid, 2);
    
    for (u, edges) in graph.adj.iter().enumerate() {
        let (x1, y1) = positions[u];
        
        for &v in edges {
            let (x2, y2) = positions[v];
            
            // Draw line
            draw::draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32);
            
            // Draw arrow head if directed
            if graph.directed {
                let angle = (y2 - y1).atan2(x2 - x1);
                let arrow_size = 10.0;
                
                // Point on circle where arrow should end
                let end_x = x2 - NODE_RADIUS as f64 * angle.cos();
                let end_y = y2 - NODE_RADIUS as f64 * angle.sin();
                
                // Arrow head points
                let arrow_angle = std::f64::consts::PI / 6.0;
                let left_x = end_x - arrow_size * (angle + arrow_angle).cos();
                let left_y = end_y - arrow_size * (angle + arrow_angle).sin();
                let right_x = end_x - arrow_size * (angle - arrow_angle).cos();
                let right_y = end_y - arrow_size * (angle - arrow_angle).sin();
                
                draw::draw_line(end_x as i32, end_y as i32, left_x as i32, left_y as i32);
                draw::draw_line(end_x as i32, end_y as i32, right_x as i32, right_y as i32);
            }
        }
    }
    
    // Draw nodes
    for (i, &(x, y)) in positions.iter().enumerate() {
        // Draw circle
        draw::set_draw_color(Color::from_rgb(70, 130, 180)); // Steel blue
        draw::draw_pie(
            x as i32 - NODE_RADIUS,
            y as i32 - NODE_RADIUS,
            NODE_RADIUS * 2,
            NODE_RADIUS * 2,
            0.0,
            360.0,
        );
        
        // Draw border
        draw::set_draw_color(Color::from_rgb(30, 60, 100));
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::draw_arc(
            x as i32 - NODE_RADIUS,
            y as i32 - NODE_RADIUS,
            NODE_RADIUS * 2,
            NODE_RADIUS * 2,
            0.0,
            360.0,
        );
        
        // Draw label
        draw::set_draw_color(Color::White);
        draw::set_font(fltk::enums::Font::HelveticaBold, 14);
        let label = i.to_string();
        let (w, h) = draw::measure(&label, true);
        draw::draw_text2(
            &label,
            x as i32 - w / 2,
            y as i32 - h / 2,
            w,
            h,
            fltk::enums::Align::Center,
        );
    }
}

pub fn show(graph: &Graph) -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    
    let mut window = Window::default()
        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_label("Hasse Diagram Visualization");
    
    let mut frame = Frame::default()
        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .center_of_parent();
    
    // Calculate positions
    let positions = calculate_layered_positions(graph);
    
    // Clone data for the closure
    let graph_clone = graph.clone();
    let positions_clone = positions.clone();
    
    frame.draw(move |_| {
        draw_graph(&graph_clone, &positions_clone);
    });
    
    window.end();
    window.show();
    
    // Add info text
    let info = format!(
        "Vertices: {} | Edges: {} | Type: {}",
        graph.num_vertices(),
        graph.num_edges(),
        if graph.directed { "Directed" } else { "Undirected" }
    );
    window.set_label(&format!("Hasse Diagram - {}", info));
    
    app.run()?;
    
    Ok(())
}

