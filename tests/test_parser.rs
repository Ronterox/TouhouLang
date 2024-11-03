use touhoulang::{parser::*, token_macro, tokenizer::Token};

token_macro!(ident, Identifier);
token_macro!(num, Number);
token_macro!(kword, Keyword);
token_macro!(poss, Possesive);
token_macro!(str, String);

macro_rules! list { ($($item: expr),*) => { Value::List(vec![$($item),*]) }; }

macro_rules! obj {
    ($id: literal, $value: expr) => {
        Object {
            id: $id.to_string(),
            value: $value,
        }
    };
}

fn expect<const N: usize, const M: usize>(tokens: [Token; N], objects: [Object; M]) {
    assert_eq!(parse(tokens.to_vec()), objects);
}

#[test]
fn parses_single_value() {
    expect(
        [ident!("age"), kword!("is"), num!(17.0)],
        [obj!("age", Value::Number(17.0))],
    );
}

#[test]
fn parses_string() {
    expect(
        [ident!("marisa"), kword!("is"), str!("marisa")],
        [obj!("marisa", Value::String("marisa".to_string()))],
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
        [
            obj!("age", Value::Number(17.0)),
            obj!("marisa", Value::Number(18.0)),
        ],
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
        [
            obj!("age", Value::Number(17.0)),
            obj!("marisa", Value::Number(17.0)),
        ],
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
        [obj!("age", Value::Number(18.0))],
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
        [
            obj!("age", Value::Number(18.0)),
            obj!("marisa", Value::Number(18.0)),
        ],
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
        [obj!("marisa", list![obj!("age", Value::Number(18.0))])],
    );

    expect(
        [ident!("marisa"), poss!("has"), ident!("age"), num!(15.0)],
        [obj!("marisa", list![obj!("age", Value::Number(15.0))])],
    );

    expect(
        [
            ident!("age"),
            poss!("of"),
            ident!("marisa"),
            kword!("is"),
            num!(15.0),
        ],
        [obj!("marisa", list![obj!("age", Value::Number(15.0))])],
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
            obj!("age", Value::Number(17.0)),
            obj!("marisa", list![obj!("age", Value::Number(17.0))]),
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
        [obj!("marisa", list![obj!("age", Value::Number(18.0))])],
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
            obj!("age", Value::Number(18.0)),
            obj!("marisa", list![obj!("age", Value::Number(18.0))]),
        ],
    );
}

