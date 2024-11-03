pub mod parser;
pub mod tokenizer;

#[macro_use]
mod evaluator;

#[cfg(test)]
mod test_integration {
    use crate::{evaluate, parser::*, tokenizer::tokenize};

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
