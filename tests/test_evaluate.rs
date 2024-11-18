use std::collections::HashMap;

use touhoulang::{evaluate, val_list, val_num, val_obj, val_str};

evaluate! {
    struct Globals {
        text: String,
        number: i32,
    }
}

evaluate! {
    struct Reimu {
        age: i32,
        items: Vec<String>,
        numbers: Vec<i32>,
    }
}

#[test]
fn evaluates_globals() {
    let objs = HashMap::from([val_str!("text", "hi mom!"), val_num!("number", 69.0)]);

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
    let objs = HashMap::from([val_obj!(
        "reimu",
        val_num!("age", 17.0),
        val_list!("items", String, "bow", "arrow"),
        val_list!("numbers", Number, 1., 2., 3.)
    )]);

    let mut res = Reimu {
        age: 0,
        items: Vec::new(),
        numbers: Vec::new(),
    };
    res.evaluate(objs);

    assert_eq!(res.age, 17);
    assert_eq!(res.items, vec!["bow".to_string(), "arrow".to_string()]);
    assert_eq!(res.numbers, vec![1, 2, 3]);
}
