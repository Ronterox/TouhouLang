#[derive(Debug, PartialEq)]
enum Token {
    Preposition(String),
    Identifier(String),
    Keyword(String),
    Integer(i32),
    Float(f32),
}

const PREPOSITIONS: [&str; 4] = ["the", "a", "an", "is"];

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

                if keywords.contains(&word.as_str()) {
                    Token::Keyword(word)
                } else if PREPOSITIONS.contains(&word.as_str()) {
                    Token::Preposition(word)
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
            _ => break,
        };
        tokens.push(token);
    }

    return tokens;
}

#[cfg(test)]
mod test_tokenizer {
    use crate::{tokenize, Token};

    struct ABC {
        a: String,
        b: bool,
        c: i32,
    }

    impl Default for ABC {
        fn default() -> Self {
            Self {
                a: "A".to_string(),
                b: true,
                c: 0,
            }
        }
    }

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
                Token::Preposition("is".to_string()),
                Token::Integer(17),
            ],
        );
    }
}
