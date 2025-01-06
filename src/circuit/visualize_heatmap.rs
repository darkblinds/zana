use plotters::prelude::*;
use std::collections::HashMap;
use num_complex::Complex;

/// Generates a heatmap visualization of the quantum statevector.
///
/// # Arguments
/// - `statevector`: A `HashMap` representing the statevector, where keys are states (e.g., `|000⟩`) and values are their complex amplitudes.
/// - `output_file`: The path to save the heatmap image.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use num_complex::Complex;
/// let statevector = HashMap::from([
///     (0, Complex::new(0.707, 0.0)),
///     (1, Complex::new(0.707, 0.0)),
/// ]);
/// visualize_heatmap(statevector, "heatmap.png");
/// ```
pub fn visualize_heatmap(statevector: HashMap<usize, Complex<f64>>, output_file: &str) {
    let probabilities: Vec<(usize, f64)> = statevector
        .iter()
        .map(|(&state, &amp)| (state, amp.norm_sqr()))
        .collect();

    // Find the number of qubits based on the largest state index
    let num_qubits = (statevector.keys().max().unwrap_or(&0) + 1)
        .next_power_of_two()
        .trailing_zeros() as usize;

    // Set up the plotting area
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Quantum State Probabilities", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..(1 << num_qubits), 0.0..1.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_desc("Probability")
        .x_desc("State (binary representation)")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    // Draw the probabilities as bars
    chart
        .draw_series(
            probabilities
                .iter()
                .map(|&(state, prob)| {
                    Rectangle::new(
                        [(state, 0.0), (state, prob)],
                        RED.filled(),
                    )
                })
        )
        .unwrap();

    println!("Heatmap saved to: {}", output_file);
}
