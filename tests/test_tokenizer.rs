use touhoulang::{token_macro, tokenizer::*};

token_macro!(ident, Identifier);
token_macro!(num, Number);
token_macro!(kword, Keyword);
token_macro!(poss, Possesive);
token_macro!(str, String);

fn expect<const N: usize>(text: &str, tokens: [Token; N]) {
    assert_eq!(tokenize(text), tokens);
}

#[test]
fn empty_is_empty_list() {
    expect("", []);
}

#[test]
fn recognizes_identifiers() {
    expect("abc", [ident!("abc")]);
}

#[test]
fn ignores_whitespace() {
    expect("    abc    ", [ident!("abc")]);
}

#[test]
fn recognizes_string() {
    expect(r#""marisa""#, [str!("marisa")]);
}

#[test]
fn recognizes_numbers() {
    expect("123", [num!(123.0)]);
    expect("1.23", [num!(1.23)]);
}

#[test]
fn recognizes_keywords() {
    expect("is marisa", [kword!("is"), ident!("marisa")]);
}

#[test]
fn excludes_prepositions() {
    expect("the a an", []);
}

#[test]
fn multiple_tokens() {
    expect("abc 123", [ident!("abc"), num!(123.0)]);

    expect(
        "reimu age is 17",
        [ident!("reimu"), ident!("age"), kword!("is"), num!(17.0)],
    );
}

#[test]
fn recognizes_posesives() {
    expect("mcdonald's", [ident!("mcdonald"), poss!("s")]);
    expect("minecraft has", [ident!("minecraft"), poss!("has")]);
    expect("age of reimu", [ident!("age"), poss!("of"), ident!("reimu")]);
}

#[test]
fn recognizes_multiple_phrases() {
    expect(
        "reimu's age is 17, and marisa's age is 18",
        [
            ident!("reimu"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            num!(18.0),
        ],
    )
}

