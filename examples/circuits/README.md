# Quantum Circuit Examples

This directory contains example programs demonstrating how to use the `QuantumCircuit` module from the Zana toolkit. These examples showcase single-qubit and multi-qubit gate operations, circuit construction, and various quantum gate applications.

## **Examples Overview**

| Example Name               | Description                                                                 |
|----------------------------|-----------------------------------------------------------------------------|
| `basic_circuit.rs`         | A basic example demonstrating the creation of a quantum circuit.           |
| `multiple_single_qubit.rs` | Demonstrates applying multiple single-qubit gates to different qubits.      |
| `multi_qubit_gates.rs`     | Shows how to add multi-qubit gates like CNOT and SWAP to a quantum circuit. |

---

## **Getting Started**

### Prerequisites

Ensure you have the following installed on your system:
- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)

### Running the Examples

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/zana.git
    cd zana
    ```

2. Navigate to the `examples/` directory:
    ```bash
    cd examples
    ```

3. Run an example using `cargo`:
    ```bash
    cargo run --example circuits
    ```

   Replace `circuits` with the desired example file name (without the `.rs` extension).

---

## **Examples in Detail**

### 1. `basic_circuit.rs`

#### Purpose
Demonstrates the creation of a simple quantum circuit with two qubits. It adds a single-qubit Hadamard gate and a two-qubit CNOT gate to the circuit.

#### Key Features
- Creates a circuit with 2 qubits.
- Adds single and multi-qubit gates.
- Prints the gates in the circuit.

#### How to Run
```bash
cargo run --example basic_circuit
```

---

### 2. `multiple_single_qubit.rs`

#### Purpose
Shows how to apply multiple single-qubit gates, including Hadamard, Pauli-X, and Pauli-Z, to different qubits in the same circuit.

#### Key Features
- Applies single-qubit gates to specific qubits.
- Demonstrates gate matrices for Hadamard, Pauli-X, and Pauli-Z.

#### Example Output
```plaintext
Circuit with multiple single-qubit gates:
Gate 1: Applied to Qubit 0, Matrix: [[(0.7071 + 0.0i), (0.7071 + 0.0i)], [(0.7071 + 0.0i), (-0.7071 + 0.0i)]]
Gate 2: Applied to Qubit 1, Matrix: [[(0.0 + 0.0i), (1.0 + 0.0i)], [(1.0 + 0.0i), (0.0 + 0.0i)]]
Gate 3: Applied to Qubit 2, Matrix: [[(1.0 + 0.0i), (0.0 + 0.0i)], [(0.0 + 0.0i), (-1.0 + 0.0i)]]
```

#### How to Run
```bash
cargo run --example multiple_single_qubit
```

---

### 3. `multi_qubit_gates.rs`

#### Purpose
Illustrates the use of multi-qubit gates, such as CNOT and SWAP, in a quantum circuit.

#### Key Features
- Adds a CNOT gate with a control and target qubit.
- Demonstrates the SWAP gate's effect on two qubits.

#### Example Output
```plaintext
Multi-Qubit Gates:
CNOT Gate: Control Qubit 0, Target Qubit 1, Matrix: [[(1.0 + 0.0i), (0.0 + 0.0i), (0.0 + 0.0i), (0.0 + 0.0i)], ...]
SWAP Gate: Control Qubit 0, Target Qubit 1, Matrix: [[(1.0 + 0.0i), (0.0 + 0.0i), (0.0 + 0.0i), (0.0 + 0.0i)], ...]
```

#### How to Run
```bash
cargo run --example multi_qubit_gates
```

---

## **Contributing**

We welcome contributions to expand the examples or improve existing ones. To contribute:
1. Fork this repository.
2. Create a feature branch:
    ```bash
    git checkout -b feature/new-example
    ```
3. Add your example in the `examples/` directory.
4. Submit a pull request with a detailed description of your changes.

---

## **License**

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for more details.

