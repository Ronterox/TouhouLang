use touhoulang::{parser, tokenizer};

fn main() {
    let filepath = std::env::args().nth(1).expect("Expected a file name");
    let text = std::fs::read_to_string(&filepath).expect("Failed to read file");

    let tokens = tokenizer::tokenize(&text);
    let objs = parser::parse(tokens);

    println!("{:#?}", objs);
}
