use slices_bitand::SlicesBitAnd;

#[test]
fn test_usize() {
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 4];
    let z = x.as_slice().bitand_as_usize(y.as_slice());
    assert_eq!(z, Some(6));
}

#[test]
fn test_usize_different_length() {
    let x = vec![1, 2];
    let y = vec![1, 2, 3];
    let z = x.as_slice().bitand_as_usize(y.as_slice());
    assert_eq!(z, None);
}

#[test]
fn test_string() {
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 4];
    let z = x.as_slice().bitand_as_string(y.as_slice());
    assert_eq!(z, Some("110".to_string()));
}

#[test]
fn test_string_different_length() {
    let x = vec![1, 2];
    let y = vec![1, 2, 3];
    let z = x.as_slice().bitand_as_string(y.as_slice());
    assert_eq!(z, None);
}
