use touhoulang_macro::Evaluate;
use touhoulang::*;

#[derive(Evaluate, Default)]
struct Reimu {
    age: i32,
}

#[test]
fn parse_derive() {
    let reimu = Reimu::new("The age of reimu is 18"); // FIX: doesn't matter capital letters
    assert_eq!(reimu.age, 18);
}
