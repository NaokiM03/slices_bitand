# slices_bitand

The slices_bitand crate provides pseudo Bitwise AND operation for slices.

## Example

```
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 4];

    let z = x.as_slice().bitand_as_usize(y.as_slice());
    assert_eq!(z, Some(6));

    let z = x.as_slice().bitand_as_string(y.as_slice());
    assert_eq!(z, Some("110".to_string()));
```
