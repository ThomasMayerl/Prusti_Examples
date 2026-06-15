// Related: pass/example6

fn exampleu3(x: u32) {
    let mut count = x;

    // an invariant would be needed here to also add constraint that count cannot be larger than x + 5
    while count < x + 5 {
        count += 1;
    }

    assert!(count == x + 5);
}