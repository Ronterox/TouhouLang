use serde::{Deserialize, Serialize};
use touhoulang::*;
use touhoulang_macro::Evaluate;

#[derive(Serialize, Deserialize, Debug)]
struct ReimuSerde {
    name: String,
    age: i32,
    damage: f32,
}

#[derive(Evaluate, Default, Debug)]
struct Reimu {
    name: String,
    age: i32,
    damage: f32,
}

fn main() {
    let serializer = std::env::args()
        .nth(1)
        .expect("Expected serializer name: 'serde' or 'touhoulang'");
    let filepath = std::env::args().nth(2).expect("Expected data file path");
    let data = std::fs::read_to_string(&filepath).expect("Failed to read data file");

    if serializer == "serde" {
        let reimu: ReimuSerde = serde_json::from_str(&data).expect("Failed to deserialize data");
        println!("{:?}", reimu);
    } else if serializer == "touhoulang" {
        let reimu = Reimu::from_str(&data);
        println!("{:?}", reimu);
    } else {
        panic!("Expected serializer name: 'serde' or 'touhoulang'");
    }
}
