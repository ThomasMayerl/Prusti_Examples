use prusti_contracts::*;

#[ensures(x.len() == old(x.len()))]
#[ensures(forall(|i: usize| (0 <= i && i < x.len()) ==> x[i] == old(x[i]) + 1) )]
fn exampleu4(x: &mut [u32]) {
    // crashes because we cannot check for permissions
    // nor do we support invariants to prove correctness if we even could prove safety
    for i in 0..x.len() {
        x[i] = x[i] + 1;
    }
}