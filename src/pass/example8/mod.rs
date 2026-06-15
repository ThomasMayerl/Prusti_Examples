use prusti_contracts::*;

trait MyTrait {
    #[ensures(result == x+1)]
    fn incr(x: i32) -> i32;
}

struct St1 {}
struct St2 {}

impl MyTrait for St1 {
    fn incr(x: i32) -> i32 {
        x + 1
    }
}

impl MyTrait for St2 {
    fn incr(x: i32) -> i32 {
        x + 2 - 1
    }
}

fn foo<T: MyTrait>() {
    assert!(T::incr(5) == 6);
}  