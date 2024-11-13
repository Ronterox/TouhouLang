use touhoulang_macro::Evaluate;
use touhoulang::*;

#[derive(Evaluate, Default)]
struct Reimu {
    age: i32,
    damage: f32,
}

#[test]
fn parse_derive() {
    let reimu = Reimu::from_str("The age of Reimu is 18, and her damage is 12.5"); // FIX: doesn't matter capital letters
    assert_eq!(reimu.age, 18);
    assert_eq!(reimu.damage, 12.5);
}
