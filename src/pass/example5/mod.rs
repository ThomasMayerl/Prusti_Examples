// Taken from Rust by Example - changed to be symbolic
// succeeds without overflow checks
// fails with overflow checks (this is expected)
fn example5(x: u32) {
    let mut count = x;

    loop {
        count += 1;

        if count == x + 3 {
            continue;
        }

        assert!(count != x + 3);

        if count == x + 5 {
            break;
        }
    }

    assert!(count == x + 5);
}