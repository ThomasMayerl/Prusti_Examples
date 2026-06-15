use prusti_contracts::*;

// Program Proofs Example 3.0

#[ensures(result == 2 * x)]
fn bad_double(x: i32) -> i32 {
    let y = bad_double(x - 1);
    y + 2
}

#[trusted]
fn non_determ() -> bool {
    panic!("Not to be called");
}

#[ensures(result == x * x)]
fn squarish(x: i32, guess: i32) -> i32 {
    if guess == x * x {
        guess
    } else if non_determ() {
        squarish(x, guess + 1)
    } else {
        squarish(x, guess - 1)
    }
}