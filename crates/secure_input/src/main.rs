use secure_input::{parse_positive_u32, read_sanitized_line, InputError};
use std::io::{self, Write};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), InputError> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut output = io::stdout().lock();

    writeln!(output, "Please enter a positive number (max 10 digits):")?;
    output.flush()?;

    let line = read_sanitized_line(&mut input, 64)?;
    let value = parse_positive_u32(&line)?;

    writeln!(output, "Sanitized number: {value}")?;
    Ok(())
}
