use clap::Parser;
use password_generator::{PasswordConfig, generate_password};

#[derive(Parser, Debug)]
#[command(name = "PasswordGenerator")]
#[command(author = "Boris K. <l0wpressu7e@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Secure password generator with customizable options")]
struct Cli {
    /// Desired password length (1-255 characters)
    #[arg(short, long, value_parser = length_in_range, default_value_t = 24)]
    length: u8,

    /// Allow numbers (0-9) in password
    #[arg(long, short)]
    numbers: bool,

    /// Allow letters (a-z, A-Z) in password
    #[arg(short = 't', long)]
    letters: bool,

    /// Allow special characters in password
    #[arg(long, short)]
    symbols: bool,
}

const LENGTH_RANGE: std::ops::RangeInclusive<usize> = 1..=255;

fn length_in_range(s: &str) -> Result<u8, String> {
    let length: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid length", s))?;

    if LENGTH_RANGE.contains(&length) {
        Ok(length as u8)
    } else {
        Err(format!(
            "Length not in range {}-{}",
            LENGTH_RANGE.start(),
            LENGTH_RANGE.end()
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = PasswordConfig::new(cli.length, cli.numbers, cli.letters, cli.symbols);
    let password = generate_password(&config);
    println!("{}", password);
    Ok(())
}
