pub mod gates;        // Expose gates.rs
pub mod statevector;

use plotters::prelude::*;
use plotters::style::Color as PlottersColor; // Avoid conflict with ratatui::Color
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::stdout;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use ratatui::text::Spans;
use ratatui::widgets::Paragraph;
use crate::circuit::gates::Gate;
use crate::circuit::statevector::Statevector;

/// Represents a quantum circuit.
///
/// A quantum circuit consists of a set number of qubits and a sequence of gate operations.
/// Single-qubit gates and multi-qubit gates are stored with their associated qubits for clarity.
pub struct QuantumCircuit {
    /// The number of qubits in the circuit.
    pub qubits: usize,

    /// A sequence of gates applied to the circuit, stored as `(gate, qubits)`.
    pub gates: Vec<(Gate, Vec<usize>)>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with the specified number of qubits.
    ///
    /// # Arguments
    /// - `qubits`: The number of qubits in the circuit.
    pub fn new(qubits: usize) -> Self {
        Self {
            qubits,
            gates: Vec::new(),
        }
    }

    /// Adds a gate to the circuit.
    ///
    /// The function dynamically determines whether the gate is single-qubit or multi-qubit
    /// based on the `Gate` enum and the number of qubits provided.
    ///
    /// # Arguments
    /// - `gate`: The gate to add (e.g., `Gate::Single` or `Gate::Two`).
    /// - `qubits`: The indices of the qubits the gate acts on.
    ///
    /// # Panics
    /// - If any qubit index is out of bounds.
    /// - If the gate size does not match the number of qubits specified.
    pub fn add_gate(&mut self, gate: Gate, qubits: Vec<usize>) {
        // Validate qubit indices
        for &qubit in &qubits {
            assert!(
                qubit < self.qubits,
                "Qubit index {} is out of bounds for a circuit with {} qubits.",
                qubit,
                self.qubits
            );
        }

        // Validate gate size
        match (&gate, qubits.len()) {
            (Gate::Single(_), 1) => (),
            (Gate::Two(_), 2) => (),
            _ => panic!("Invalid gate or mismatched qubits for gate type."),
        }

        self.gates.push((gate, qubits));
    }

    /// Simulates the quantum circuit and returns the final statevector.
    ///
    /// # Returns
    /// - A `Statevector` representing the quantum system's state after all gates have been applied.
    ///
    /// # Panics
    /// - If the circuit contains invalid gates or qubit indices.
    pub fn simulate(&self) -> Statevector {
        let mut statevector = Statevector::new(self.qubits);
        for (gate, qubits) in &self.gates {
            statevector.apply_gate(gate.clone(), qubits.as_slice()); // Clone the gate
        }
        statevector
    }

    /// Visualizes the quantum circuit as a text-based diagram.
    ///
    /// # How It Works
    /// - Single-qubit gates are represented by their symbols (e.g., `H` for Hadamard).
    /// - Multi-qubit gates use `●` for control qubits and `⊕` for target qubits.
    /// - The visualization includes all qubits and the sequence of gates applied to them.
    ///
    /// # Example Output
    /// For a circuit with a Hadamard on Q0 and a CNOT (control: Q0, target: Q1):
    /// ```
    /// Q0: ───H───●─────
    ///             │
    /// Q1: ───────⊕─────
    /// ```
    ///
    /// # Example Usage
    /// ```
    /// use zana::circuit::{gates, QuantumCircuit};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.add_gate(gates::hadamard(), vec![0]);
    /// circuit.add_gate(gates::cnot(), vec![0, 1]);
    ///
    /// circuit.visualize();
    /// ```
    pub fn visualize(&self) {
        let mut layers: Vec<String> = vec![String::new(); self.qubits];

        for (gate, qubits) in &self.gates {
            match gate {
                Gate::Single(_) => {
                    let qubit = qubits[0];
                    layers[qubit].push_str("──H──"); // Replace "H" for specific gates
                }
                Gate::Two(_) => {
                    let control = qubits[0];
                    let target = qubits[1];
                    for (i, layer) in layers.iter_mut().enumerate() {
                        if i == control {
                            layer.push_str("──●──");
                        } else if i == target {
                            layer.push_str("──⊕──");
                        } else {
                            layer.push_str("─────");
                        }
                    }
                }
            }
        }

        for (i, layer) in layers.iter().enumerate() {
            println!("Q{}: {}", i, layer);
        }
    }

    pub fn visualize_heatmap(&self, output_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let final_state = self.simulate();

        let probabilities: Vec<(usize, f64)> = final_state
            .vector
            .iter()
            .map(|(&state, &amp)| (state, amp.norm_sqr()))
            .collect();


        match output_file {
            Some(file) => {
                let root = BitMapBackend::new(file, (800, 600)).into_drawing_area();
                self.draw_heatmap(root, &probabilities)?;
                println!("Heatmap saved to: {}", file);
            }
            None => {
                self.render_heatmap_in_terminal(&probabilities)?;
            }
        }

        Ok(())
    }

    fn draw_heatmap<DB>(
        &self,
        root: DrawingArea<DB, plotters::coord::Shift>,
        probabilities: &[(usize, f64)],
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        DB: DrawingBackend,
        DB::ErrorType: 'static, // Ensure the error type satisfies 'static lifetime
    {
        for &(state, prob) in probabilities {
            println!("DEBUG: State: {}, Probability: {}", state, prob);
        }

        root.fill(&WHITE)?;
        let max_prob = probabilities.iter().map(|&(_, prob)| prob).fold(0.0, f64::max);
        let y_axis_max = (max_prob * 1.2).max(0.1); // Ensures bars are visible

        let mut chart = ChartBuilder::on(&root)
            .caption("Quantum State Probabilities", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..(1 << self.qubits) as i32, 0.0..y_axis_max)?;

        chart
            .configure_mesh()
            .x_labels(10)
            .y_desc("Probability")
            .x_desc("State (binary representation)")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart
            .draw_series(
                probabilities
                    .iter()
                    .map(|&(state, prob)| Rectangle::new([(state as i32, 0.0), (state as i32, prob)], RED.filled())),
            )?;

        root.present()?;
        Ok(())
    }

    pub fn render_heatmap_in_terminal(
        &self,
        probabilities: &[(usize, f64)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();

        // Enter alternate screen
        execute!(stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Normalize probabilities to scale them dynamically
        let max_prob = probabilities.iter().map(|&(_, prob)| prob).fold(0.0, f64::max);
        let scale_factor = if max_prob > 0.0 { 100.0 / max_prob } else { 1.0 }; // Normalize to 100 if max_prob > 0

        let bar_data: Vec<(String, u64)> = probabilities
            .iter()
            .map(|&(state, prob)| {
                let scaled_value = (prob * scale_factor).ceil() as u64;
                let state_binary = format!("{:08b}", state);
                (state_binary, scaled_value.max(1))
            })
            .collect();

        // Convert to reference-based data for the bar chart
        let bar_data_refs: Vec<(&str, u64)> = bar_data
            .iter()
            .map(|(state, prob)| (state.as_str(), *prob))
            .collect();

        // Navigation state
        let mut selected_index = 0;
        let mut start_index = 0; // First visible index

        loop {
            // Render UI
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Percentage(80), Constraint::Min(3)].as_ref())
                    .split(frame.size());

                // Header
                let header = Paragraph::new(Spans::from("Quantum Heatmap Visualization"))
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::BOTTOM).title("Header"));

                // Highlight logic: Adjust data styling
                let bar_data_with_highlight: Vec<(&str, u64)> = bar_data_refs
                    .iter()
                    .enumerate()
                    .map(|(index, &(label, value))| {
                        if index == selected_index {
                            (label, value * 2) // Emphasize the selected bar
                        } else {
                            (label, value)
                        }
                    })
                    .collect();

                // Heatmap
                let bar_chart = BarChart::default()
                    .block(Block::default().title("Quantum State Probabilities").borders(Borders::ALL))
                    .bar_width(3)
                    .bar_gap(1)
                    .style(Style::default().fg(Color::LightBlue))
                    .value_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                    .label_style(Style::default().fg(Color::Gray))
                    .data(&bar_data_with_highlight);

                let footer = Paragraph::new(vec![
                    Spans::from(format!(
                        "Selected State: {} | Probability: {:.2}%",
                        bar_data_refs[selected_index].0,
                        probabilities[selected_index].1 * 100.0, // Display the probability in %
                    )),
                    Spans::from("Press 'q' or 'Esc' to exit"), // Add the helpful message
                ])
                    .style(Style::default().fg(Color::Green))
                    .block(Block::default().borders(Borders::TOP).title("Footer"));

                frame.render_widget(header, chunks[0]);
                frame.render_widget(bar_chart, chunks[1]);
                frame.render_widget(footer, chunks[2]);
            })?;

            // Handle user input
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < bar_data_refs.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break, // Exit on 'q' or Esc
                    _ => {}
                }
            }
        }

        // Leave alternate screen
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
