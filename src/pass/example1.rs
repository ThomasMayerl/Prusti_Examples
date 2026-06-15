use prusti_contracts::*;

fn example1() {
    assert!(fibonacci(0) == 0);
    assert!(fibonacci(1) == 1);
    assert!(fibonacci(2) == 1);
    assert!(fibonacci(3) == 2);
    assert!(fibonacci(4) == 3);
    //prusti_assert!(fibonacci(5) == 5);
    assert!(fibonacci(5) == 5);
}

#[pure]
#[requires(n >= 0)]
fn fibonacci_spec(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_spec(n - 1) + fibonacci_spec(n - 2),
    }
    
}

#[requires(n >= 0)]
#[ensures(result == fibonacci_spec(n))]
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}