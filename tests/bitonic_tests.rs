use code_kata_bitonic_sequence_rust::get_bitonic_sequence_v2;

#[test]
fn test_case1() {
    let seq = get_bitonic_sequence_v2(5, 3, 10);
    assert_eq!(seq, vec![9, 10, 9, 8, 7]);
}

#[test]
fn test_case2() {
    let seq = get_bitonic_sequence_v2(7, 2, 5);
    assert_eq!(seq, vec![2, 3, 4, 5, 4, 3, 2]);
}

#[test]
fn test_case3_impossible() {
    let seq = get_bitonic_sequence_v2(5, 7, 8);
    assert_eq!(seq, vec![-1]);
}

#[test]
fn test_case2_min_size() {
    let seq = get_bitonic_sequence_v2(1, 3, 10);
    assert_eq!(seq, vec![9]); 
}

