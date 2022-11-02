use std::char::from_u32;
use std::ops::{Range, RangeInclusive};
use rand::{Rng, thread_rng};
use clap::Parser;

#[derive(Parser)]
#[command(name = "PasswordGenerator")]
#[command(author = "Boris K. <l0wpressu7e@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Password-generator", long_about = None)]
struct Cli {
    /// Desired password length
    #[arg(value_parser = length_in_range, default_value_t = 24)]
    length: u8,
    /// Allow numbers in password
    #[arg(long, short)]
    numbers: bool,
    /// Allow letters in password
    #[arg(long, short)]
    letters: bool,
    /// Allow special characters in password
    #[arg(long, short)]
    symbols: bool,
}


fn main() {
    let cli = Cli::parse();

    let mut need_numbers = cli.numbers;
    let mut need_letters = cli.letters;
    let mut need_symbols = cli.symbols;

    if !cli.numbers && !cli.letters && !cli.symbols {
        need_numbers = true;
        need_letters = true;
        need_symbols = true;
    };

    let mut result = String::new();
    let num_range: [Range<u32>; 1] = [48..57];
    let sym_range: [Range<u32>; 2] = [58..64, 91..96];
    let let_range: [Range<u32>; 2] = [65..90, 97..122];

    for _ in 0..cli.length {
        let mut types_amount: u8 = 0;
        let mut current_symbol: Vec<char> = vec![];

        if need_numbers {
            current_symbol.push(generate_symbol(&num_range[0]));
            types_amount += 1;
        }
        if need_letters {
            let rng = thread_rng().gen_range(0..=1);
            current_symbol.push(generate_symbol(&let_range[rng]));
            types_amount += 1;
        }
        if need_symbols {
            let rng = thread_rng().gen_range(0..=1);
            current_symbol.push(generate_symbol(&sym_range[rng]));
            types_amount += 1;
        }
        let rng: usize = thread_rng().gen_range(0..types_amount) as usize;
        let s = current_symbol[rng];

        result.push(s);
    }

    println!("{}", result);
}

const LENGTH_RANGE: RangeInclusive<usize> = 1..=255;

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

fn generate_symbol(range: &Range<u32>) -> char {
    from_u32(thread_rng().gen_range(range.to_owned())).unwrap()
}
