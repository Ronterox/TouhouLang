use core::panic;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Preposition(String),
    Possesive(String),
    Identifier(String),
    Keyword(String),
    Integer(i32),
    Float(f32),
    None,
}

const PREPOSITIONS: [&str; 3] = ["the", "a", "an"];
const BUILT_IN_KEYWORDS: [&str; 1] = ["is"];
const POSESIVES: [&str; 2] = ["s", "of"];

#[allow(dead_code)]
fn tokenize(text: &str, keywords: &[&str]) -> Vec<Token> {
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

                if PREPOSITIONS.contains(&word.as_str()) {
                    Token::Preposition(word)
                } else if keywords.contains(&word.as_str())
                    || BUILT_IN_KEYWORDS.contains(&word.as_str())
                {
                    Token::Keyword(word)
                } else if POSESIVES.contains(&word.as_str()) {
                    Token::Possesive(word)
                } else {
                    Token::Identifier(word)
                }
            }
            '0'..='9' => {
                let condition = |c: &char| c.is_digit(10) || *c == '.';
                let number = String::from_iter(chars.by_ref().take_while(condition));
                if number.contains('.') {
                    Token::Float(number.parse().unwrap())
                } else {
                    Token::Integer(number.parse().unwrap())
                }
            }
            _ => panic!("Unexpected character: {c}"),
        };
        tokens.push(token);
    }

    return tokens;
}

#[derive(Debug, PartialEq)]
enum Value {
    Object(Token),
    Single(Token),
}

#[derive(Debug, PartialEq)]
struct Data {
    id: String,
    value: Value,
    args: HashMap<String, Value>,
}

#[allow(dead_code)]
fn parse(tokens: Vec<Token>) -> Vec<Data> {
    let mut data = vec![];
    let mut tok = tokens
        .iter()
        .filter(|t| !matches!(t, Token::Preposition(_)));
    while let (Some(a), Some(b), Some(c)) = (tok.next(), tok.next(), tok.next()) {
        let parse_result = match (a, b, c) {
            (Token::Identifier(a), Token::Keyword(_), Token::Keyword(c)) => Data {
                id: a.to_string(),
                value: Value::Object(Token::Keyword(c.to_string())),
                args: HashMap::new(),
            },
            (Token::Identifier(a), Token::Keyword(_), c) => Data {
                id: a.to_string(),
                value: Value::Single(c.clone()),
                args: HashMap::new(),
            },
            (Token::Identifier(a), Token::Possesive(_), Token::Identifier(c)) => {
                match (
                    tok.next().unwrap_or(&Token::None),
                    tok.next().unwrap_or(&Token::None),
                ) {
                    (Token::Keyword(_), value) => Data {
                        id: a.to_string(),
                        value: Value::Object(Token::None),
                        args: HashMap::from([(c.to_string(), Value::Single(value.clone()))]),
                    },
                    _ => panic!("Unexpected token: {a:?} {b:?} {c:?}"),
                }
            }
            _ => panic!("Unexpected token: {a:?} {b:?} {c:?}"),
        };
        data.push(parse_result);
    }
    return data;
}

#[cfg(test)]
mod test_tokenizer {
    use crate::{tokenize, Token};

    const KEYWORDS: [&str; 2] = ["reimu", "marisa"];

    fn expect<const N: usize>(text: &str, tokens: [Token; N]) {
        assert_eq!(tokenize(text, &KEYWORDS), tokens);
    }

    #[test]
    fn empty_is_empty_list() {
        expect("", []);
    }

    #[test]
    fn recognizes_identifiers() {
        expect("abc", [Token::Identifier("abc".to_string())]);
    }

    #[test]
    fn ignores_whitespace() {
        expect("    abc    ", [Token::Identifier("abc".to_string())]);
    }

    #[test]
    fn recognizes_numbers() {
        expect("123", [Token::Integer(123)]);
        expect("1.23", [Token::Float(1.23)]);
    }

    #[test]
    fn recognizes_keywords() {
        expect(
            "reimu marisa",
            [
                Token::Keyword("reimu".to_string()),
                Token::Keyword("marisa".to_string()),
            ],
        );
    }

    #[test]
    fn recognizes_prepositions() {
        expect(
            "the a an",
            [
                Token::Preposition("the".to_string()),
                Token::Preposition("a".to_string()),
                Token::Preposition("an".to_string()),
            ],
        );
    }

    #[test]
    fn multiple_tokens() {
        expect(
            "abc 123",
            [Token::Identifier("abc".to_string()), Token::Integer(123)],
        );

        expect(
            "reimu age is 17",
            [
                Token::Keyword("reimu".to_string()),
                Token::Identifier("age".to_string()),
                Token::Keyword("is".to_string()),
                Token::Integer(17),
            ],
        );
    }

    #[test]
    fn recognizes_posesives() {
        expect(
            "mcdonald's",
            [
                Token::Identifier("mcdonald".to_string()),
                Token::Possesive("s".to_string()),
            ],
        );
    }
}

#[cfg(test)]
mod test_parser {
    use std::collections::HashMap;

    use crate::{parse, Data, Token, Value};

    fn expect<const N: usize, const M: usize>(input: [Token; N], output: [Data; M]) {
        assert_eq!(parse(input.to_vec()), output);
    }

    #[test]
    fn no_tokens_is_empty() {
        expect([], []);
    }

    #[test]
    fn parses_object() {
        expect(
            [
                Token::Identifier("reimu".to_string()),
                Token::Keyword("is".to_string()),
                Token::Preposition("the".to_string()),
                Token::Keyword("player".to_string()),
            ],
            [Data {
                id: "reimu".to_string(),
                value: Value::Object(Token::Keyword("player".to_string())),
                args: HashMap::new(),
            }],
        );

        expect(
            [
                Token::Identifier("reimu".to_string()),
                Token::Possesive("s".to_string()),
                Token::Identifier("age".to_string()),
                Token::Keyword("is".to_string()),
                Token::Integer(17),
            ],
            [Data {
                id: "reimu".to_string(),
                value: Value::Object(Token::None),
                args: HashMap::from([("age".to_string(), Value::Single(Token::Integer(17)))]),
            }],
        );
    }

    #[test]
    fn parses_value() {
        expect(
            [
                Token::Identifier("age".to_string()),
                Token::Keyword("is".to_string()),
                Token::Integer(17),
            ],
            [Data {
                id: "age".to_string(),
                value: Value::Single(Token::Integer(17)),
                args: HashMap::new(),
            }],
        );
    }
}
