// Related: unsupported/exampleu7

use prusti_contracts::*;

// Note: Crashes if this struct is called Str
struct MyStruct {
    x: i32,
    y: i32
}

fn example7() {
    let mut val = MyStruct {
        x: 5,
        y: 5
    };
    let x = get_x(&mut val);
    *x = 6;
    assert!(val.x == 6);
    // assert!(val.y == 5);  we cannot check for that, see unsupported/exampleu7
}

#[after_expiry(val.x == before_expiry(*result))]
fn get_x(val: &mut MyStruct) -> &mut i32 {
    &mut val.x
}