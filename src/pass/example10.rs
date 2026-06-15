// This test case somehow only works when calling Viper on the encoded Viper program
// Prusti says it cannot verify the postcondition for me

use prusti_contracts::*;

#[requires(!(exists(|i: usize| 0 <= i && i < x.len() && x[i] != 5)))]
#[ensures(forall(|i: usize| (0 <= i && i < x.len()) ==> x[i] == 5))]
fn example10(x: &[u32]) {}