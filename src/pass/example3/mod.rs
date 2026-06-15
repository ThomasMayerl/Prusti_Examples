// Related: unsupported/exampleu1

fn example3() {
    let x = 2.5;
    let x = x * 2.0;
    let x = x % 3.0;
    let x = x - 1.5;
    let x = x + 2.5;
    //let x = x as i32 - 1;
    assert!(x == 3.0);
}