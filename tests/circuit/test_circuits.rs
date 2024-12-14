use zana::circuit::{gates, QuantumCircuit};

#[test]
fn test_circuit_with_gates() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.add_gate(gates::hadamard());
    circuit.add_gate(gates::cnot());

    assert_eq!(circuit.gates, vec!["H", "CNOT"]);
}
