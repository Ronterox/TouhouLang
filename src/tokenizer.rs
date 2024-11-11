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
        $list.contains(&$word.to_lowercase().as_str())
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

