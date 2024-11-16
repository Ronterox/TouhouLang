use std::collections::HashMap;

use touhoulang::{parser::*, token_macro, tokenizer::Token, val_num, val_obj, val_str};

token_macro!(ident, Identifier);
token_macro!(num, Number);
token_macro!(kword, Keyword);
token_macro!(poss, Possesive);
token_macro!(str, String);

fn expect<const N: usize, const M: usize>(tokens: [Token; N], result: [(String, Value); M]) {
    let result = HashMap::from(result);
    let expected = parse(tokens.to_vec());
    for key in result.keys() {
        assert_eq!(expected.get(key).unwrap(), result.get(key).unwrap());
    }
}

#[test]
fn parses_single_value() {
    expect(
        [ident!("age"), kword!("is"), num!(17.0)],
        [val_num!("age", 17.0)],
    );
}

#[test]
fn parses_string() {
    expect(
        [ident!("marisa"), kword!("is"), str!("marisa")],
        [val_str!("marisa", "marisa")],
    );
}

#[test]
fn parses_multiple_values() {
    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            kword!("is"),
            num!(18.0),
        ],
        [val_num!("age", 17.0), val_num!("marisa", 18.0)],
    );
}

#[test]
fn parses_variable() {
    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            kword!("is"),
            ident!("age"),
        ],
        [val_num!("age", 17.0), val_num!("marisa", 17.0)],
    );
}

#[test]
fn updates_by_variable() {
    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("age"),
            kword!("is"),
            num!(18.0),
        ],
        [val_num!("age", 18.0)],
    );

    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(18.0),
            //
            ident!("marisa"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            kword!("is"),
            ident!("age"),
        ],
        [val_num!("age", 18.0), val_num!("marisa", 18.0)],
    );
}

#[test]
fn parses_attribute() {
    expect(
        [
            ident!("marisa"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            num!(18.0),
        ],
        [val_obj!("marisa", val_num!("age", 18.0))],
    );

    expect(
        [ident!("marisa"), poss!("has"), ident!("age"), num!(15.0)],
        [val_obj!("marisa", val_num!("age", 15.0))],
    );

    expect(
        [
            ident!("age"),
            poss!("of"),
            ident!("marisa"),
            kword!("is"),
            num!(15.0),
        ],
        [val_obj!("marisa", val_num!("age", 15.0))],
    );
}

#[test]
fn parses_attribute_variable() {
    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            ident!("age"),
        ],
        [
            val_num!("age", 17.0),
            val_obj!("marisa", val_num!("age", 17.0)),
        ],
    );
}

#[test]
fn updates_attribute() {
    expect(
        [
            ident!("marisa"),
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
        [val_obj!("marisa", val_num!("age", 18.0))],
    );

    expect(
        [
            ident!("age"),
            kword!("is"),
            num!(18.0),
            //
            ident!("marisa"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            num!(17.0),
            //
            ident!("marisa"),
            poss!("s"),
            ident!("age"),
            kword!("is"),
            ident!("age"),
        ],
        [
            val_num!("age", 18.0),
            val_obj!("marisa", val_num!("age", 18.0)),
        ],
    );
}
