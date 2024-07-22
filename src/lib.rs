mod macros;

use macros::concat;
use std::collections::HashMap;

const NUM_KEYWORDS: [&str; 29] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
];

const CONTROL_KEYWORDS: [&str; 12] = [
    "if", "else", "is", "it", "and", "with", "when", "to", "can", "in", "out", "on",
];
const POSITION_KEYWORDS: [&str; 6] = ["up", "down", "left", "right", "top", "bottom"];
const POSSESIVE_KEYWORDS: [&str; 3] = ["its", "has", "of"];
const OBJECT_KEYWORDS: [&str; 2] = ["enemy", "player"];

const KEYWORDS: [&str; 23] = concat_array!(
    CONTROL_KEYWORDS,
    POSITION_KEYWORDS,
    POSSESIVE_KEYWORDS,
    OBJECT_KEYWORDS
);

const GLOBAL_METHODS: [&str; 1] = ["display"];

#[derive(Debug, Clone)]
enum Token {
    Method(String),
    Property(String),
    Variable(String),
    Keyword(String),
    Number(String),
    List(String),
    Text(String),
}

fn parse_num_word(word: &str) -> String {
    match word {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "ten" => "10",
        "eleven" => "11",
        "twelve" => "12",
        "thirteen" => "13",
        "fourteen" => "14",
        "fifteen" => "15",
        "sixteen" => "16",
        "seventeen" => "17",
        "eighteen" => "18",
        "nineteen" => "19",
        "twenty" => "20",
        "thirty" => "30",
        "forty" => "40",
        "fifty" => "50",
        "sixty" => "60",
        "seventy" => "70",
        "eighty" => "80",
        "ninety" => "90",
        "hundred" => "100",
        _ => word,
    }
    .to_string()
}

fn tokenize_word(word_chars: &mut Vec<char>, context: char, tokens: &mut Vec<Token>) -> char {
    let word: String = word_chars.iter().collect();
    word_chars.clear();

    let word = match word.trim() {
        "the" | "then" | "a" | "an" | "at" => "",
        "are" => "is",
        _ => word.trim(),
    };

    if word.is_empty() {
        return context;
    }

    let token: Token;
    if context == ')' {
        token = Token::List(word.to_string());
    } else if context == '"' {
        token = Token::Text(word.to_string());
    } else if context == 's' {
        if word == "'s" {
            return context;
        }
        token = Token::Property(word.to_string());
    } else if GLOBAL_METHODS.contains(&word) {
        token = Token::Method(word.to_string());
    } else if KEYWORDS.contains(&word) {
        token = Token::Keyword(word.to_string());
        if word == "of" {
            if let Some(Token::Variable(varname)) = tokens.last() {
                *tokens.last_mut().unwrap() = Token::Property(varname.to_owned());
            }
        }
    } else if word.chars().all(char::is_numeric) || NUM_KEYWORDS.contains(&word) {
        token = Token::Number(parse_num_word(&word));
    } else if word.ends_with('s') {
        token = Token::Method(word.to_string());
    } else {
        token = Token::Variable(word.to_string());
    }

    tokens.push(token);
    char::default()
}

fn tokenize_line(line: &str, tokens: &mut Vec<Token>) {
    let mut word_chars = Vec::<char>::new();
    let mut context = char::default();
    let mut chars = line.chars().enumerate();

    while let Some((i, c)) = chars.next() {
        if c.is_whitespace() && !context.is_ascii_punctuation() || context == c {
            word_chars.push(c);
            context = tokenize_word(&mut word_chars, context, tokens);
            continue;
        } else if c.is_ascii_punctuation() && !context.is_ascii_punctuation() {
            if !word_chars.is_empty() {
                context = tokenize_word(&mut word_chars, context, tokens);
            }

            if c == ',' {
                tokenize_line(line.split_at(i + 1).1, tokens);
                break;
            }

            context = match c {
                '(' => ')',
                '"' => '"',
                '\'' => 's',
                _ => char::default(),
            };
        }
        word_chars.push(c);
    }

    if !word_chars.is_empty() {
        tokenize_word(&mut word_chars, context, tokens);
    }
}

fn tokenize_text(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    text.split(".\n")
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            tokenize_line(line, &mut tokens);
        });
    tokens
}

fn parse_word_value(token: Token, variables: &HashMap<String, String>) -> String {
    match token {
        Token::Variable(varname) => variables.get(&varname).unwrap_or(&varname).to_owned(),
        Token::Keyword(varname) => varname,
        Token::Number(num) => num,
        Token::Text(text) => text,
        Token::List(list) => list,
        _ => "".to_owned(),
    }
}

fn next_token(tokens: &mut impl Iterator<Item = Token>, error: &str) -> Token {
    tokens.next().expect(format!("expected {error}").as_str())
}

fn update_variable(
    name: String,
    error: &str,
    variables: &mut HashMap<String, String>,
    tokens: &mut impl Iterator<Item = Token>,
) {
    let token = next_token(tokens, &error);
    let value = parse_word_value(token, variables);

    if OBJECT_KEYWORDS.contains(&value.as_str()) {
        variables.insert(value, name);
    } else {
        variables.insert(name, value);
    }
}

// TODO: Generic parsing for actual rust types, so it outputs them out
pub fn parse_text(text: &str) -> (HashMap<String, String>, Vec<(String, String)>) {
    let tokens = tokenize_text(text);

    let mut tokens = tokens.into_iter();
    let mut actions: Vec<(String, String)> = Vec::new();
    let mut variables: HashMap<String, String> = HashMap::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Variable(name) => match next_token(&mut tokens, "token after variable") {
                Token::Keyword(keyword) if keyword == "is" => update_variable(
                    name.to_owned(),
                    "value after 'is'",
                    &mut variables,
                    &mut tokens,
                ),
                Token::Property(property) => match next_token(&mut tokens, "keyword 'is'") {
                    Token::Keyword(keyword) if keyword == "is" => update_variable(
                        format!("{name}.{property}"),
                        "value after 'is'",
                        &mut variables,
                        &mut tokens,
                    ),
                    _ => {}
                },
                Token::Method(method) => {
                    actions.push((name.to_owned(), method.to_owned()));
                }
                _ => {}
            },
            Token::Property(property) => match next_token(&mut tokens, "keyword 'of'") {
                Token::Keyword(keyword) if keyword == "of" => {
                    match next_token(&mut tokens, "variable after 'of'") {
                        Token::Variable(name) => match next_token(&mut tokens, "keyword 'is'") {
                            Token::Keyword(keyword) if keyword == "is" => update_variable(
                                format!("{name}.{property}"),
                                "value after 'is'",
                                &mut variables,
                                &mut tokens,
                            ),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            },
            Token::Keyword(name) if OBJECT_KEYWORDS.contains(&name.as_str()) => {
                match next_token(&mut tokens, "keyword 'is'") {
                    Token::Keyword(keyword) if keyword == "is" => {
                        let token = next_token(&mut tokens, "value after 'is'");
                        let value = parse_word_value(token, &variables);
                        variables.insert(name, value);
                    }
                    _ => {}
                }
            }
            _ => (),
        }
    }

    (variables, actions)
}

#[cfg(test)]
mod test {
    use crate::parse_text;
    use std::collections::HashMap;

    fn expect<const N: usize>(text: &str, result: [(&str, &str); N]) {
        let (mut vars, _) = parse_text(text);
        let vars: HashMap<&str, &str> = vars
            .iter_mut()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();

        dbg!(&vars);
        assert_eq!(vars, HashMap::from(result));
    }

    fn expect_fail(text: &str) {
        assert!(std::panic::catch_unwind(|| parse_text(text)).is_err());
    }

    #[test]
    fn test_keyword_is() {
        expect("reimu is 18", [("reimu", "18")]);
        expect("reimu is 18, reimu is reimu", [("reimu", "18")]);

        expect(
            "marissa is 18, reimu is marissa",
            [("reimu", "18"), ("marissa", "18")],
        );

        expect(
            "reimu is 18 and the person is reimu",
            [("reimu", "18"), ("person", "18")],
        );

        expect("chan is twelve", [("chan", "12")]);

        let text = "\"A donburin\"";
        expect(format!("reimu is {text}").as_str(), [("reimu", text)]);
    }

    #[test]
    fn test_properties() {
        let text = "\"A mage\"";
        expect(
            format!("marissa is {text}, reimu's donburin is a marissa").as_str(),
            [("marissa", text), ("reimu.donburin", text)],
        );

        expect("reimu's donburin is 10", [("reimu.donburin", "10")]);
        expect(
            "the person's pattern is (left right right left)",
            [("person.pattern", "(left right right left)")],
        );

        expect(
            "the health of marissa is a hundred",
            [("marissa.health", "100")],
        );

        expect("marissa's bullets are 20", [("marissa.bullets", "20")]);

        expect_fail("marissa has 20 health");
        expect_fail("marissa's health is 20, marissa has 1 health");

        expect("reimu's bullet speed is 20", [("reimu.bullet.speed", "20")]);
        expect("the bullet speed of reimu is 10", [("reimu.bullet.speed", "10")]);
    }

    #[test]
    fn test_object_keywords() {
        expect(
            "the player is reimu, the enemy is marissa",
            [("player", "reimu"), ("enemy", "marissa")],
        );

        expect(
            "reimu is the player, and marissa is the enemy",
            [("player", "reimu"), ("enemy", "marissa")],
        );
    }
}
