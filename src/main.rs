use std::env::args;

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

const KEYWORDS: [&str; 12] = [
    "if", "else", "is", "it", "and", "with", "when", "to", "can", "in", "out", "on",
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

fn parse_word(word_chars: &mut Vec<char>, context: char) -> char {
    let word: String = word_chars.iter().collect();
    word_chars.clear();

    let word = match word.trim() {
        "the" | "named" | "then" | "a" | "an" | "at" => "",
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

    println!("{:?}", token);
    char::default()
}

fn parse_line(line: &str) {
    let mut word_chars = Vec::<char>::new();
    let mut context = char::default();
    let mut chars = line.chars().enumerate();

    while let Some((i, c)) = chars.next() {
        if c.is_whitespace() && !context.is_ascii_punctuation() || context == c {
            context = parse_word(&mut word_chars, context);
        } else if c.is_ascii_punctuation() && !context.is_ascii_punctuation() {
            if !word_chars.is_empty() {
                context = parse_word(&mut word_chars, context);
            }

            if c == ',' {
                parse_line(line.split_at(i + 1).1);
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
        parse_word(&mut word_chars, context);
    }
}

fn parse_text(text: &String) {
    text.split(".\n")
        .filter(|line| line.len() > 0)
        .enumerate()
        .for_each(|(i, line)| {
            println!("\nLine {}: {}\nParsed:", i + 1, line);
            parse_line(line);
        });
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if let Some(main_file) = args.get(1) {
        let file = std::fs::read_to_string(main_file)?;
        parse_text(&file);
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

    #[test]
    fn test_main_missing_filepath() {
        let result = super::main();
        assert!(result.is_err());
    }
}
