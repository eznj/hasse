// src/lib.rs
// Library entry point for the Hasse diagram tool

pub mod graph;
pub mod gui;

// Re-export commonly used types
pub use graph::Graph;
