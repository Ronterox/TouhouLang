use std::{collections::HashMap, env::args};

const NUM_KEYWORDS: [&str; 26] = [
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
];

const KEYWORDS: [&str; 13] = [
    "if", "else", "is", "it", "and", "with", "when", "to", "can", "in", "out", "on", "named",
];

const POSITION_KEYWORDS: [&str; 6] = ["up", "down", "left", "right", "top", "bottom"];
const POSSESIVE_KEYWORDS: [&str; 3] = ["its", "has", "of"];

const GLOBAL_METHODS: [&str; 1] = ["display"];

#[derive(Debug)]
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
        _ => word,
    }
    .to_string()
}

fn tokenize_word(word_chars: &mut Vec<char>, context: char, tokens: &mut Vec<Token>) -> char {
    let word: String = word_chars.iter().collect();
    word_chars.clear();

    let word = match word.trim() {
        "the" | "then" | "a" | "an" | "at" => "",
        _ => word.trim(),
    };

    if word.is_empty() && context == char::default() {
        return context;
    }

    let token: Token;
    if context == ')' {
        token = Token::List(word.to_string());
    } else if context == '"' {
        token = Token::Text(word.to_string());
    } else if context == 's' {
        if word.is_empty() {
            return context;
        } else {
            token = Token::Property(word.to_string());
        }
    } else if GLOBAL_METHODS.contains(&word) {
        token = Token::Method(word.to_string());
    } else if KEYWORDS.contains(&word)
        || POSSESIVE_KEYWORDS.contains(&word)
        || POSITION_KEYWORDS.contains(&word)
    {
        token = Token::Keyword(word.to_string());
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
            context = tokenize_word(&mut word_chars, context, tokens);
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
        } else {
            word_chars.push(c);
        }
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

fn parse_text(text: &str) -> (HashMap<String, String>, Vec<(String, String)>) {
    let tokens = tokenize_text(text);
    let mut tokens = tokens.iter();

    let mut actions: Vec<(&String, &String)> = Vec::new();
    let mut variables: HashMap<&str, &str> = HashMap::new();

    while let Some(token) = tokens.next() {
        if let Token::Variable(name) = token {
            match tokens.next().expect("expected token after variable") {
                Token::Keyword(keyword) => match keyword.as_str() {
                    "is" => {
                        let value = match tokens.next().expect("expected value after 'is'") {
                            Token::Variable(varname) => variables
                                .get(varname.as_str())
                                .expect("expected variable value"),
                            Token::Text(text) => text.as_str(),
                            Token::Keyword(kword) => match kword.as_str() {
                                "named" => {
                                    match tokens.next().expect("expected name after 'named'") {
                                        Token::Variable(name) => name,
                                        _ => "",
                                    }
                                }
                                _ => "",
                            },
                            Token::Number(num) => num,
                            _ => "",
                        };
                        variables.insert(name, value);
                    }
                    _ => {}
                },
                Token::Method(method) => {
                    actions.push((name, method));
                }
                _ => {}
            }
        }
    }
    let vars: HashMap<String, String> = variables
        .iter_mut()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
    let actions: Vec<(String, String)> = actions
        .iter_mut()
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect();

    (vars, actions)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if let Some(main_file) = args.get(1) {
        let file = std::fs::read_to_string(main_file)?;
        let (_, _) = parse_text(&file);
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No main file provided",
        ))?
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::parse_text;

    #[test]
    fn test_keyword_is() {
        let expect = |text: &str, result: (&str, &str)| {
            let (mut vars, _) = parse_text(text);
            let vars: HashMap<&str, &str> = vars
                .iter_mut()
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect();

            dbg!(&vars);
            assert!(vars == HashMap::from([result]));
        };
        expect("reimu is 18", ("reimu", "18"));
    }
}
