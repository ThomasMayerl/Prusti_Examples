// Related: pass/example1

fn exampleu1() {
    let x = 2.5;
    let x = x * 2.0;
    let x = x % 3.0;
    let x = x - 1.5;
    let x = x + 2.5;
    let x = x as i32 - 1; // cast unsupported
    assert!(x == 2);
}