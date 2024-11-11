use touhoulang::{evaluate, parser::*};

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

