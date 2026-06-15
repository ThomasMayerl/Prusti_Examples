use prusti_contracts::*;

enum MyEnum {
    Var1,
    Var2(i32, i32),
    Var3 {x: i64, y: i64}
}

#[requires(
    match(val) {
        MyEnum::Var2(x, y) => x == 5 && y == 2,
        MyEnum::Var3{x, y} => x == 2 && y == 5,
        _ => true
    }
)]
fn example11(val: MyEnum) {
    if let MyEnum::Var2(x, y) = val {
        assert!(x + y == 7);
    }
    if let MyEnum::Var3{x, y} = val {
        assert!(x + y == 7);
    }
}