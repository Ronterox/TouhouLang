#[derive(Debug, PartialEq, Clone)]
enum Token {
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
                prep!("and"),
                ident!("marisa"),
                poss!("s"),
                ident!("age"),
                kword!("is"),
                num!(18.0),
            ],
        )
    }
}

macro_rules! find {
    ($list: ident, $name: ident) => {
        $list
            .iter()
            .find(|o| o.id == *$name)
            .expect("Found object")
            .value
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
    let mut objects = vec![];

    while let (Some(a), Some(b), Some(c)) = (tokens.next(), tokens.next(), tokens.next()) {
        match (a, b, c) {
            (Token::Identifier(name), Token::Keyword(k), Token::Number(value)) if k == "is" => {
                objects.push(Object {
                    id: name.to_string(),
                    value: Value::Number(*value as f32),
                })
            }
            (Token::Identifier(name), Token::Keyword(k), Token::Identifier(value)) if k == "is" => {
                objects.push(Object {
                    id: name.to_string(),
                    value: find!(objects, value).clone(),
                })
            }
            (Token::Identifier(name), Token::Possesive(k), Token::Identifier(property))
                if k == "s" =>
            {
                match (tokens.next().unwrap(), tokens.next().unwrap()) {
                    (Token::Keyword(k), Token::Number(value)) if k == "is" => {
                        objects.push(Object {
                            id: name.to_string(),
                            value: Value::List(vec![Object {
                                id: property.to_string(),
                                value: Value::Number(*value as f32),
                            }]),
                        })
                    }
                    (Token::Keyword(k), Token::Identifier(variable)) if k == "is" => {
                        objects.push(Object {
                            id: name.to_string(),
                            value: Value::List(vec![Object {
                                id: property.to_string(),
                                value: find!(objects, variable).clone(),
                            }]),
                        })
                    }
                    (d, e) => panic!("Unexpected tokens: ->{:?}<-", [a, b, c, d, e]),
                }
            }
            _ => panic!("Unexpected tokens: ->{:?}<-", [a, b, c]),
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
    fn parse_single_value() {
        expect(
            [ident!("age"), kword!("is"), num!(17.0)],
            [obj!("age", Value::Number(17.0))],
        );
    }

    #[test]
    fn parse_multiple_values() {
        expect(
            [
                ident!("age"),
                kword!("is"),
                num!(17.0),
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
    fn parse_variable() {
        expect(
            [
                ident!("age"),
                kword!("is"),
                num!(17.0),
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
    fn parse_posessive_s() {
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
    fn parse_posessive_variable() {
        expect(
            [
                ident!("age"),
                kword!("is"),
                num!(17.0),
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
}
