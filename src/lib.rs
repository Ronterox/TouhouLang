#[derive(Debug, PartialEq, Clone)]
enum Token {
    String(String),
    Preposition(String),
    Possesive(String),
    Identifier(String),
    Keyword(String),
    Number(f32),
    None,
}

#[macro_export]
macro_rules! token_macro {
    ($id: ident, Number) => {
        macro_rules! $id {
            ($name: literal) => {
                Token::Number($name)
            };
        }
    };
    ($id: ident, $token: ident) => {
        macro_rules! $id {
            ($name: literal) => {
                Token::$token($name.to_string())
            };
        }
    };
}

const PREPOSITIONS: [&str; 4] = ["the", "a", "an", "and"];
const KEYWORDS: [&str; 1] = ["is"];
const POSSESIVES: [&str; 2] = ["s", "of"];

macro_rules! contains {
    ($list: ident, $word: ident) => {
        $list.contains(&$word.as_str())
    };
}

#[allow(dead_code)]
fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = text
        .trim()
        .chars()
        .skip_while(|c| c.is_whitespace())
        .peekable();

    while let Some(c) = chars.peek() {
        let token = match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let condition = |c: &char| c.is_alphanumeric() || *c == '_';
                let word = String::from_iter(chars.by_ref().take_while(condition));

                if contains!(PREPOSITIONS, word) {
                    Token::Preposition(word)
                } else if contains!(KEYWORDS, word) {
                    Token::Keyword(word)
                } else if contains!(POSSESIVES, word) {
                    Token::Possesive(word)
                } else {
                    Token::Identifier(word)
                }
            }
            '0'..='9' => {
                let condition = |c: &char| c.is_digit(10) || *c == '.';
                let number = String::from_iter(chars.by_ref().take_while(condition));
                Token::Number(number.parse().expect("Correct number format"))
            }
            '"' => {
                chars.next();
                let string = String::from_iter(chars.by_ref().take_while(|c| *c != '"'));
                Token::String(string)
            }
            ' ' | ',' => {
                chars.next();
                Token::None
            }
            _ => panic!("Unexpected character: ->{c}<-"),
        };
        tokens.push(token);
    }

    return tokens.into_iter().filter(|t| *t != Token::None).collect();
}

#[cfg(test)]
mod test_tokenizer {
    use crate::{tokenize, Token};

    token_macro!(ident, Identifier);
    token_macro!(num, Number);
    token_macro!(kword, Keyword);
    token_macro!(poss, Possesive);
    token_macro!(prep, Preposition);
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
    fn recognizes_prepositions() {
        expect("the a an", [prep!("the"), prep!("a"), prep!("an")]);
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
                prep!("and"),
                //
                ident!("marisa"),
                poss!("s"),
                ident!("age"),
                kword!("is"),
                num!(18.0),
            ],
        )
    }
}

macro_rules! find_mut_obj {
    ($list: ident, $name: ident) => {
        $list.iter_mut().find(|o| o.id == *$name)
    };
}

macro_rules! find_obj {
    ($list: ident, $name: ident) => {
        $list.iter().find(|o| o.id == *$name)
    };
}

macro_rules! find_value {
    ($list: ident, $name: ident) => {
        find_obj!($list, $name).expect("Found object").value
    };
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Number(f32),
    String(String),
    List(Vec<Object>),
}

#[derive(Debug, PartialEq, Clone)]
struct Object {
    id: String,
    value: Value,
}

#[allow(dead_code)]
fn parse(tokens: Vec<Token>) -> Vec<Object> {
    let mut tokens = tokens.iter();
    let mut objects = Vec::<Object>::new();

    while let (Some(a), Some(b), Some(c)) = (tokens.next(), tokens.next(), tokens.next()) {
        match (a, b, c) {
            // ident! kword! num!
            (Token::Identifier(name), Token::Keyword(k), Token::Number(value)) if k == "is" => {
                let value = Value::Number(*value);

                if let Some(obj) = find_mut_obj!(objects, name) {
                    obj.value = value;
                } else {
                    objects.push(Object {
                        id: name.to_string(),
                        value,
                    });
                }
            }
            // ident! kword! str!
            (Token::Identifier(name), Token::Keyword(k), Token::String(value)) if k == "is" => {
                let value = Value::String(value.to_string());

                if let Some(obj) = find_mut_obj!(objects, name) {
                    obj.value = value;
                } else {
                    objects.push(Object {
                        id: name.to_string(),
                        value,
                    });
                }
            }
            // ident! kword! ident!
            (Token::Identifier(name), Token::Keyword(k), Token::Identifier(var)) if k == "is" => {
                let value = find_value!(objects, var).clone();

                if let Some(obj) = find_mut_obj!(objects, name) {
                    obj.value = value;
                } else {
                    objects.push(Object {
                        id: name.to_string(),
                        value,
                    });
                }
            }
            // ident! poss! ident!
            (Token::Identifier(name), Token::Possesive(k), Token::Identifier(property))
                if k == "s" =>
            {
                match (tokens.next().unwrap(), tokens.next().unwrap()) {
                    (Token::Keyword(k), Token::Number(value)) if k == "is" => {
                        let value = Value::Number(*value);

                        match find_mut_obj!(objects, name) {
                            Some(obj) => match &mut obj.value {
                                Value::List(list) => match find_mut_obj!(list, property) {
                                    Some(obj) => obj.value = value,
                                    None => list.push(Object {
                                        id: property.to_string(),
                                        value,
                                    }),
                                },
                                _ => panic!("Expected {name} to be a list of args"),
                            },
                            None => objects.push(Object {
                                id: name.to_string(),
                                value: Value::List(vec![Object {
                                    id: property.to_string(),
                                    value,
                                }]),
                            }),
                        }
                    }
                    (Token::Keyword(k), Token::Identifier(var)) if k == "is" => {
                        let value = find_value!(objects, var).clone();

                        match find_mut_obj!(objects, name) {
                            Some(obj) => match &mut obj.value {
                                Value::List(list) => match find_mut_obj!(list, property) {
                                    Some(obj) => obj.value = value,
                                    None => list.push(Object {
                                        id: property.to_string(),
                                        value,
                                    }),
                                },
                                _ => panic!("Expected {name} to be a list of args"),
                            },
                            None => objects.push(Object {
                                id: name.to_string(),
                                value: Value::List(vec![Object {
                                    id: property.to_string(),
                                    value,
                                }]),
                            }),
                        }
                    }
                    (d, e) => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c, d, e]),
                }
            }
            _ => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c]),
        }
    }

    objects
}

#[cfg(test)]
mod test_parser {
    use crate::{parse, Object, Token, Value};

    token_macro!(ident, Identifier);
    token_macro!(num, Number);
    token_macro!(kword, Keyword);
    token_macro!(poss, Possesive);
    token_macro!(str, String);

    macro_rules! list {
        ($($item: expr),*) => {
            Value::List(vec![$($item),*])
        };
    }

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
}

#[macro_export]
macro_rules! evaluate {
    ($objs: ident, $struct: ident) => {{
        let name = std::any::type_name::<$struct>();
        if name.ends_with("::Globals") {
            $struct::default()
        } else {
            $struct::default()
        }
    }};
}

#[cfg(test)]
mod evaluate_test {
    use crate::{Object, Value};

    macro_rules! obj {
        ($id: literal, $value: expr) => {
            Object {
                id: $id.to_string(),
                value: $value,
            }
        };
    }

    #[derive(Default)]
    struct Globals {
        text: String,
    }

    #[test]
    fn evaluates_string() {
        let _objs = vec![obj!("text", Value::String("hi mom!".to_string()))];
        let res = evaluate!(objs, Globals);
        assert_eq!(res.text, "hi mom!");
    }
}
