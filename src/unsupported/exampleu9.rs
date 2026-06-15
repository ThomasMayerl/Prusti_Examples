// Related: unsupported/exampleu8

fn exampleu9() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    assert!(v.len() == 3);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
}