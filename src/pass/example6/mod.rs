// Related: unsupported/exampleu3

// succeeds without overflow checks
// fails with overflow checks (this is expected)
fn example6(x: u32) {
    let mut count = x;

    while count < x + 5 {
        count += 1;
    }

    assert!(count >= x + 5);
}