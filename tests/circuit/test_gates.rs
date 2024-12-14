use zana::circuit::gates;

#[test]
fn test_hadamard_gate() {
    assert_eq!(gates::hadamard(), "H");
}

#[test]
fn test_cnot_gate() {
    assert_eq!(gates::cnot(), "CNOT");
}
