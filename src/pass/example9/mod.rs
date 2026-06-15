use prusti_contracts::*;

fn example9() {
    let mut x = 5;
    no_change(&mut x);
    assert!(x == 5);
}

#[ensures(*x == old(*x))]
fn no_change(x: &mut i32) {

}