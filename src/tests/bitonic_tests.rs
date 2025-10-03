use code_kata_bitonic_sequence_rust::get_bitonic_sequence;

#[test]
fn test_case1() {
    let seq = get_bitonic_sequence(5, 3, 10);
    assert_eq!(seq, vec![9, 10, 9, 8, 7]);
}

#[test]
fn test_case2_min_size() {
    let seq = get_bitonic_sequence(1, 3, 10);
    assert_eq!(seq, vec![9]); // só o primeiro elemento
}

