use test_vectors::test_vectors;

#[test_vectors(dir = "tests/basic")]
fn replace_spaces_with_underscore(input: &[u8], expected: &[u8]) {
    let instr = std::str::from_utf8(input).unwrap();
    let expstr = std::str::from_utf8(expected).unwrap();
    let output = instr.replace(' ', "_");
    assert_eq!(expstr, &output);
}
