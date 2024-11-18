pub mod parser;
pub mod tokenizer;

#[macro_use]
mod evaluator;

#[cfg(test)]
mod test_integration {
    use crate::evaluate;

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
            items: Vec<String>,
        }
    }

    #[test]
    fn integrates_from_start_to_finish() {
        let input = r#"
        the age is 17, and item is "Minecraft".
        the age of marisa is 18, and reimu's age is age, also reimu has an item item
        "#;

        let globals = Globals::from_str(input);
        let reimu = Reimu::from_str(input);
        let marisa = Marisa::from_str(input);

        assert_eq!(globals.age, 17);

        assert_eq!(reimu.age, 17);
        assert_eq!(reimu.item, "Minecraft");

        assert_eq!(marisa.age, 18);
    }
}
