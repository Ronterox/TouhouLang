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

