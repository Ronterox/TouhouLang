use std::env::args;
use touhoulang::parse_text;

fn main() -> std::io::Result<()> {
    if let Some(main_file) = args().collect::<Vec<String>>().get(1) {
        let file_text = std::fs::read_to_string(main_file)?;
        let (_, _) = parse_text(&file_text);
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No main file provided",
        ))?
    }

    Ok(())
}
