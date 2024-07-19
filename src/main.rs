use std::{env::args, ops::ControlFlow};

fn parse_word(word_chars: &mut Vec<char>) {
    let word: String = word_chars.iter().collect();

    let word = match word.trim() {
        "the" | "named" => "",
        _ => &word,
    };

    if !word.is_empty() {
        println!("{}", word);
    }

    word_chars.clear();
}

fn parse_line(line: &str) {
    let mut word_chars = Vec::<char>::new();
    let mut context = char::default();

    line.chars().enumerate().try_for_each(|(i, c)| {
        if c.is_whitespace() && !context.is_ascii_punctuation() || context == c {
            if context == c {
                context = char::default();
                word_chars.push(c);
            }

            if !word_chars.is_empty() {
                parse_word(&mut word_chars);
                return ControlFlow::Continue(());
            }
        } else if c.is_ascii_punctuation() && !context.is_ascii_punctuation() {
            if c == ',' {
                if !word_chars.is_empty() {
                    parse_word(&mut word_chars);
                }
                parse_line(line.split_at(i + 1).1);
                return ControlFlow::Break(());
            }

            context = match c {
                '(' => ')',
                '"' => '"',
                _ => char::default(),
            };
        }

        word_chars.push(c);
        ControlFlow::Continue(())
    });

    if !word_chars.is_empty() {
        parse_word(&mut word_chars);
    }
}

fn parse_file(file: &String) {
    file.split(".\n")
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
        parse_file(&file);
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
