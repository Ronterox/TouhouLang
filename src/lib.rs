pub mod tokenizer {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
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

    const PREPOSITIONS: [&str; 5] = ["the", "a", "an", "and", "also"];
    const KEYWORDS: [&str; 1] = ["is"];
    const POSSESIVES: [&str; 4] = ["s", "of", "has", "have"];

    macro_rules! contains {
        ($list: ident, $word: ident) => {
            $list.contains(&$word.as_str())
        };
    }

    pub fn tokenize(text: &str) -> Vec<Token> {
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
                c if c.is_whitespace() || c.is_ascii_punctuation() => {
                    chars.next();
                    Token::None
                }
                _ => panic!("Unexpected character: ->{c}<-"),
            };
            tokens.push(token);
        }

        return tokens
            .into_iter()
            .filter(|t| match t {
                Token::None | Token::Preposition(_) => false,
                _ => true,
            })
            .collect();
    }
}

#[cfg(test)]
mod test_tokenizer {
    use crate::{token_macro, tokenizer::*};

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
}

pub mod parser {
    use crate::tokenizer::Token;

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
            find_obj!($list, $name).expect("Didn't find object").value
        };
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Value {
        Number(f32),
        String(String),
        List(Vec<Object>),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Object {
        pub id: String,
        pub value: Value,
    }

    pub fn parse(tokens: Vec<Token>) -> Vec<Object> {
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
                (Token::Identifier(name), Token::Keyword(k), Token::Identifier(var))
                    if k == "is" =>
                {
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
                    let value = match (tokens.next().unwrap(), tokens.next().unwrap()) {
                        (Token::Keyword(k), Token::Number(value)) if k == "is" => {
                            Value::Number(*value)
                        }
                        (Token::Keyword(k), Token::String(value)) if k == "is" => {
                            Value::String(value.to_string())
                        }
                        (Token::Keyword(k), Token::Identifier(var)) if k == "is" => {
                            find_value!(objects, var).clone()
                        }
                        (d, e) => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c, d, e]),
                    };

                    if let Some(obj) = find_mut_obj!(objects, name) {
                        match &mut obj.value {
                            Value::List(list) => match find_mut_obj!(list, property) {
                                Some(obj) => obj.value = value,
                                None => list.push(Object {
                                    id: property.to_string(),
                                    value,
                                }),
                            },
                            _ => panic!("Expected {name} to be a list of args"),
                        }
                    } else {
                        objects.push(Object {
                            id: name.to_string(),
                            value: Value::List(vec![Object {
                                id: property.to_string(),
                                value,
                            }]),
                        });
                    }
                }
                // ident! poss! ident!
                (Token::Identifier(name), Token::Possesive(k), Token::Identifier(property))
                    if k == "has" || k == "have" =>
                {
                    let value = match tokens.next().unwrap() {
                        Token::Number(value) => Value::Number(*value),
                        Token::String(value) => Value::String(value.to_string()),
                        Token::Identifier(var) => find_value!(objects, var).clone(),
                        _ => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c]),
                    };

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
                // ident! poss! ident!
                (Token::Identifier(property), Token::Possesive(k), Token::Identifier(name))
                    if k == "of" =>
                {
                    // TODO: Macro his
                    let value = match (tokens.next().unwrap(), tokens.next().unwrap()) {
                        (Token::Keyword(k), Token::Number(value)) if k == "is" => {
                            Value::Number(*value)
                        }
                        (Token::Keyword(k), Token::String(value)) if k == "is" => {
                            Value::String(value.to_string())
                        }
                        (Token::Keyword(k), Token::Identifier(var)) if k == "is" => {
                            find_value!(objects, var).clone()
                        }
                        (d, e) => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c, d, e]),
                    };

                    // TODO: And macro this
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
                _ => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c]),
            }
        }

        objects
    }
}

#[cfg(test)]
mod test_parser {
    use crate::{parser::*, token_macro, tokenizer::Token};

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
}

#[macro_use]
mod evaluator {
    #[allow(unused_macros)]
    macro_rules! parse_number {
        ($value: expr, $num: ty) => {
            match $value {
                Value::String(s) => s.parse().expect("Expected number"),
                Value::Number(n) => n as $num,
                _ => panic!("Expected string or number"),
            }
        };
    }

    #[allow(unused_macros)]
    macro_rules! parse_value {
        ($value: expr, String) => {
            match $value {
                Value::String(s) => s,
                Value::Number(n) => n.to_string(),
                _ => panic!("Expected string or number"),
            }
        };
        ($value: expr, i32) => {
            parse_number!($value, i32)
        };
        ($value: expr, f32) => {
            parse_number!($value, f32)
        };
    }

    #[allow(unused_macros)]
    macro_rules! impl_evaluate {
        (Globals, $($field_name:ident: $field_type:tt,)*) => {
            impl Globals {
                pub fn evaluate(&mut self, objs: Vec<Object>) {
                    for obj in objs {
                        match obj.id.as_str() {
                            $(stringify!($field_name) => {
                                self.$field_name = parse_value!(obj.value, $field_type);
                            })*
                            _ => {},
                        }
                    }
                }
            }
        };
        ($name: ident, $($field_name:ident: $field_type:tt,)*) => {
            impl $name {
                pub fn evaluate(&mut self, objs: Vec<Object>) {
                    if let Some(obj) = objs.iter().find(|o| o.id == stringify!($name).to_lowercase()) {
                        if let Value::List(list) = &obj.value {
                            for obj in list {
                                match obj.id.as_str() {
                                    $(stringify!($field_name) => {
                                        self.$field_name = parse_value!(obj.value.clone(), $field_type);
                                    })*
                                    _ => {},
                                }
                            }
                        } else {
                            panic!("Expected {} to be a list of args", stringify!($name));
                        }
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! evaluate {
        (
        $(#[$doc:meta])*
        struct $name: ident {
            $($field_name:ident: $field_type:tt,)*
        }
        ) => {
            $(#[$doc])*
            #[derive(Default)]
            struct $name {
                $($field_name: $field_type,)*
            }

            impl_evaluate!($name, $($field_name: $field_type,)*);

            impl $name {
                #[allow(dead_code)]
                fn evaluate_text(&mut self, text: &str) {
                    self.evaluate(parse(tokenize(text)));
                }

                #[allow(dead_code)]
                fn new(code: &str) -> Self {
                    let mut me = Self::default();
                    me.evaluate_text(code);
                    me
                }
            }
        }
    }
}

#[cfg(test)]
mod test_evaluate {
    use crate::{parser::*, tokenizer::tokenize, evaluate};

    macro_rules! list { ($($item: expr),*) => { Value::List(vec![$($item),*]) }; }

    macro_rules! obj {
        ($id: literal, $value: expr) => {
            Object {
                id: $id.to_string(),
                value: $value,
            }
        };
    }

    evaluate! {
        struct Globals {
            text: String,
            number: i32,
        }
    }

    evaluate! {
        struct Reimu {
            age: i32,
        }
    }

    #[test]
    fn evaluates_globals() {
        let objs = vec![
            obj!("text", Value::String("hi mom!".to_string())),
            obj!("number", Value::Number(69.0)),
        ];

        let mut res = Globals {
            text: String::new(),
            number: 0,
        };

        res.evaluate(objs);

        assert_eq!(res.text, "hi mom!");
        assert_eq!(res.number, 69);
    }

    #[test]
    fn evaluates_objects() {
        let objs = vec![obj!("reimu", list![obj!("age", Value::Number(17.0))])];

        let mut res = Reimu { age: 0 };
        res.evaluate(objs);

        assert_eq!(res.age, 17);
    }
}

#[cfg(test)]
mod test_integration {
    use crate::{parser::*, tokenizer::tokenize, evaluate};

    evaluate! {
        struct Globals {
            age: i32,
        }
    }

    evaluate! {
        struct Reimu {
            age: i32,
            item: String,
        }
    }

    evaluate! {
        struct Marisa {
            age: i32,
        }
    }

    #[test]
    fn integrates_from_start_to_finish() {
        let input = r#"
        the age is 17, and item is "Minecraft".
        the age of marisa is 18, and reimu's age is age, also reimu has an item item
        "#;

        let globals = Globals::new(input);
        let reimu = Reimu::new(input);
        let marisa = Marisa::new(input);

        assert_eq!(globals.age, 17);

        assert_eq!(reimu.age, 17);
        assert_eq!(reimu.item, "Minecraft");

        assert_eq!(marisa.age, 18);
    }
}
