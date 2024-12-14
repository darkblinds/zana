# Circuit Module: Quantum Circuit Simulator

The Circuit module in Zana serves as the foundation for simulating **quantum circuits**, enabling users to construct and experiment with the basic building blocks of quantum computing. This module provides tools to define circuits with qubits, apply quantum gates, and explore fundamental quantum concepts like superposition and entanglement.

---

## **Purpose**

The Circuit module's purpose is to:

- Simulate quantum circuits with qubits and gates.
- Enable exploration of quantum principles through an intuitive interface.
- Act as a foundational layer for more complex quantum operations, such as cryptographic protocols or algorithm simulations.
- Provide tools for developers and researchers new to quantum computing.

---

## **Features**

### **Implemented Features**
- Define circuits with a user-defined number of qubits.
- Apply basic quantum gates:
    - **Hadamard (H)**: Creates superposition.
    - **CNOT**: Entangles qubits.
    - **Pauli Gates (X, Z)**: Apply standard quantum operations.
- Manipulate gates and inspect circuit configurations.
- Unit and integration tests to ensure reliability.

### **Planned Features**
1. **Advanced Gate Support**:
    - Add parameterized gates, such as rotation gates (`Rx`, `Ry`, `Rz`).
    - Support user-defined gates.

2. **Statevector Simulation**:
    - Track the quantum state of the circuit.
    - Simulate measurement and state collapse.

3. **Visualization Tools**:
    - Provide textual or graphical representations of circuits.
    - Example:
      ```
      |0> --- H --- CNOT ---
      |0> -----------●------
      ```

4. **Noise and Error Modeling**:
    - Simulate decoherence and other real-world imperfections.

5. **Integration with Other Modules**:
    - Cryptographic protocols and gameplay mechanics.

---

## **How to Use**

[//]: # (TODO: add how to use it as package once Rust Cargo is oficially published )

### **Setup**
Clone the repository and navigate to the `circuit/` module:
```bash
git clone https://github.com/your-username/zana.git
cd zana/src/circuit
```

### **Example: Building a Circuit**
Here's how to define a simple circuit and apply gates:

```rust
use zana::circuit::{gates, QuantumCircuit};

fn main() {
    let mut circuit = QuantumCircuit::new(2); // Create a circuit with 2 qubits
    circuit.add_gate(gates::hadamard());      // Apply Hadamard gate
    circuit.add_gate(gates::cnot());          // Apply CNOT gate

    println!("Quantum Circuit: {:?}", circuit.gates);
}
```

### **Run Tests**
Ensure the module works as expected:
```bash
cargo test --test circuit
```

---

## **Roadmap**

### **Short-Term Goals**
- Implement rotation gates.
- Add support for user-defined gates.
- Create an educational demo for quantum concepts like superposition.

### **Long-Term Goals**
- Full statevector simulation.
- Integration with cryptographic protocols and other modules.
- Visualization tools for debugging and education.

---

## **Folder Structure**
```
circuit/
├── circuit.rs          # Main module for defining circuits
├── gates.rs            # Module for defining gates
├── tests/              # Unit tests for the circuit module
│   ├── test_circuit.rs
│   ├── test_gates.rs
└── examples/           # Examples demonstrating circuit usage
    ├── circuit_demo.rs
```

---

## **Contributing**

We welcome contributions! Here's how you can help:

1. Fork the repository.
2. Create a feature branch:
   ```bash
   git checkout -b feature/circuit-enhancements
   ```
3. Add your code and tests.
4. Submit a pull request with a clear description of the changes.

---

## **License**

This module is licensed under the Mozilla Public License Version 2.0. See the `LICENSE` file for more details.

