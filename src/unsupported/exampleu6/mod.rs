use prusti_contracts::*;

fn exampleu6() {
    let mut x = 6;
    let mut y = 3;
    let a = return_larger(&mut x, &mut y);
    *a = 8;
    assert!(x == 8);
    assert!(y == 3);

    let mut x = 6;
    let mut y = 10;
    let a = return_larger(&mut x, &mut y);
    *a = 8;
    assert!(x == 6);
    assert!(y == 8);
}

#[ensures(*x == old(*x))]
#[ensures(*y == old(*y))]
#[ensures(old(*x > *y) ==> result === x)] // === somehow doesn't seem to work right now
#[ensures(old(!(*x > *y)) ==> result === y)]
fn return_larger<'a>(x: &'a mut i32, y: &'a mut i32) -> &'a mut i32 {
    if *x > *y {
        &mut (*x)
    } else {
        &mut (*y)
    }
}