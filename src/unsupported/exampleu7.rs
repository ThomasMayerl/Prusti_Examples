// Related: pass/example7

use prusti_contracts::*;

// Note: Crashes if this struct is called Str
struct MyStruct {
    x: i32,
    y: i32
}

fn exampleu7() {
    let mut val = MyStruct {
        x: 5,
        y: 5
    };
    let x = get_x(&mut val);
    *x = 6;
    assert!(val.x == 6);
    assert!(val.y == 5);
}

#[after_expiry(val.x == before_expiry(*result))]
#[after_expiry(val.y == old(val.y))] // this leads to a permission error
fn get_x(val: &mut MyStruct) -> &mut i32 {
    &mut val.x
}