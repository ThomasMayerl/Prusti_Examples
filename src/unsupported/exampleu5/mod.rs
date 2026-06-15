fn exampleu5() {
    // closures not supported
    let incr = |i: u32| i+1;
    assert!(incr(5) == 6);
}