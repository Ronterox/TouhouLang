use std::collections::HashMap;

use crate::tokenizer::Token;

#[macro_export]
macro_rules! val_str {
    ($name: literal, $str: literal) => {
        (
            $name.to_string(),
            $crate::parser::Value::String($str.to_string()),
        )
    };
}

#[macro_export]
macro_rules! val_num {
    ($name: literal, $num: literal) => {
        ($name.to_string(), $crate::parser::Value::Number($num))
    };
}

#[macro_export]
macro_rules! val_obj {
    ($name: literal, $tuple: expr) => {
        (
            $name.to_string(),
            $crate::parser::Value::Object(HashMap::from([$tuple])),
        )
    };
}

#[macro_export]
macro_rules! val_list {
    ($name: literal, $value: ident, $($list: expr),*) => {
        (
            $name.to_string(),
            $crate::parser::Value::List([$($crate::parser::Value::$value($list.into())),*].to_vec()),
        )
    };
}

#[macro_export]
macro_rules! set_obj_property {
    ($result: ident, $name: ident, $property: ident, $value: ident) => {
        if let Some(obj) = $result.get_mut(&$name.to_lowercase()) {
            match obj {
                Value::Object(map) => map.insert($property.to_string(), $value),
                tt => panic!("Expected {} to be a object but found {tt:?}!", $name),
            }
        } else {
            $result.insert(
                $name.to_string(),
                Value::Object(HashMap::from([($property.to_string(), $value)])),
            )
        };
    };
}

pub type Object = std::collections::HashMap<String, Value>;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f32),
    String(String),
    List(Vec<Value>),
    Object(Object),
}

pub fn parse(tokens: Vec<Token>) -> Object {
    let mut tokens = tokens.iter();
    let mut result = HashMap::<String, Value>::new();

    while let (Some(a), Some(b), Some(c)) = (tokens.next(), tokens.next(), tokens.next()) {
        match (a, b, c) {
            // ident! kword! num!
            (Token::Identifier(name), Token::Keyword(k), Token::Number(value)) if k == "is" => {
                let value = Value::Number(*value);
                result.insert(name.to_lowercase(), value);
            }
            // ident! kword! str!
            (Token::Identifier(name), Token::Keyword(k), Token::String(value)) if k == "is" => {
                let value = Value::String(value.to_string());
                result.insert(name.to_lowercase(), value);
            }
            // ident! kword! ident!
            (Token::Identifier(name), Token::Keyword(k), Token::Identifier(var)) if k == "is" => {
                let value = result.get(var).expect(&format!("Didn't find {var}"));
                result.insert(name.to_lowercase(), value.clone());
            }
            // ident! poss! ident!
            (Token::Identifier(name), Token::Possesive(k), Token::Identifier(property))
                if k == "s" =>
            {
                let value = match (tokens.next().unwrap(), tokens.next().unwrap()) {
                    (Token::Keyword(k), Token::Number(value)) if k == "is" => Value::Number(*value),
                    (Token::Keyword(k), Token::String(value)) if k == "is" => {
                        Value::String(value.to_string())
                    }
                    (Token::Keyword(k), Token::Identifier(var)) if k == "is" => result
                        .get(var)
                        .expect(&format!("Didn't find {var}"))
                        .clone(),
                    (d, e) => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c, d, e]),
                };

                set_obj_property!(result, name, property, value);
            }
            // ident! poss! ident!
            (Token::Identifier(name), Token::Possesive(k), Token::Identifier(property))
                if k == "has" || k == "have" =>
            {
                let value = match tokens.next().unwrap() {
                    Token::Number(value) => Value::Number(*value),
                    Token::String(value) => Value::String(value.to_string()),
                    Token::Identifier(var) => result
                        .get(&var.to_lowercase())
                        .expect(&format!("Didn't find {var}"))
                        .clone(),
                    _ => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c]),
                };

                set_obj_property!(result, name, property, value);
            }
            // ident! poss! ident!
            (Token::Identifier(property), Token::Possesive(k), Token::Identifier(name))
                if k == "of" =>
            {
                // TODO: Macro his
                let value = match (tokens.next().unwrap(), tokens.next().unwrap()) {
                    (Token::Keyword(k), Token::Number(value)) if k == "is" => Value::Number(*value),
                    (Token::Keyword(k), Token::String(value)) if k == "is" => {
                        Value::String(value.to_string())
                    }
                    (Token::Keyword(k), Token::Identifier(var)) if k == "is" => result
                        .get(var)
                        .expect(format!("Didn't find {var}").as_str())
                        .clone(),
                    (d, e) => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c, d, e]),
                };

                set_obj_property!(result, name, property, value);
            }
            _ => panic!("Unexpected token pattern: ->{:?}<-", [a, b, c]),
        }
    }

    result
}
