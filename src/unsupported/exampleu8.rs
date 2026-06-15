// Related: unsupported/exampleu9

fn exampleu8() {
    let v = vec!(1, 2, 3); // this does not work
    assert!(v.len() == 3);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
}