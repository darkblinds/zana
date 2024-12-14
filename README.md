# zana
Zana on Sol - an open source Quantum processor toolkit built in Rust

Documentations still on the run - check later!


### CONCEPTS:
Measurement: Collapses the quantum state into a definite classical result.
Superposition: Allows qubits to exist in a combination of states, enabling parallelism.
Entanglement: Correlates qubits, enabling powerful quantum communication and computation.
Bloch Sphere: A visual tool to understand qubit states and transformations.


TODO:
- this cover first simple architectures quantum processor executes one circuit at a timetectures
  - parallel architectures to be implemented 


# Roadmap: TO BE IMPORTED
1. STATEVECTOR
   The statevector represents the quantum state. Let’s focus on:

Normalization:
Ensure the sum of squared amplitudes equals 1 after every operation.
Validation:
Add functions to check if the statevector is valid (e.g., properly normalized, correct dimensions).
Separation of Concerns:
Ensure it handles only the quantum state (no gate logic or circuits).
- add a normalize function
- add a validate function
- add tests for these utilities
2. GATES
   Focus on a consistent representation for gates.
Action Plan
   Use a Gate enum or struct to unify gate representations.
   Include metadata (e.g., name, matrix, required qubits).
   Ensure gates are reusable and immutable.
3. CIRCUITS
   Ensure circuits abstract operations efficiently.

Action Plan
Refactor QuantumCircuit to store operations (e.g., Gate + qubits).
Add a function to apply gates to the Statevector.