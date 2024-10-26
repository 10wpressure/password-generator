use std::ops::Range;

#[derive(Debug)]
pub struct PasswordConfig {
    pub length: u8,
    pub char_ranges: Vec<Range<u32>>,
}

/// Character ranges for different types of symbols
pub const NUM_RANGES: [Range<u32>; 1] = [48..58];
pub const LETTER_RANGES: [Range<u32>; 2] = [65..91, 97..123];
pub const SYMBOL_RANGES: [Range<u32>; 4] = [33..48, 58..65, 91..97, 123..127];

impl PasswordConfig {
    pub fn new(length: u8, numbers: bool, letters: bool, symbols: bool) -> Self {
        let mut char_ranges = Vec::new();

        let (need_numbers, need_letters, need_symbols) = if !numbers && !letters && !symbols {
            (true, true, true)
        } else {
            (numbers, letters, symbols)
        };

        if need_numbers {
            char_ranges.extend_from_slice(&NUM_RANGES);
        }
        if need_letters {
            char_ranges.extend_from_slice(&LETTER_RANGES);
        }
        if need_symbols {
            char_ranges.extend_from_slice(&SYMBOL_RANGES);
        }

        Self {
            length,
            char_ranges,
        }
    }

    pub fn validate_password(&self, password: &str) -> bool {
        if password.len() != self.length as usize {
            return false;
        }

        let has_number = password.chars().any(|c| c.is_numeric());
        let has_letter = password.chars().any(|c| c.is_alphabetic());
        let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

        match (self.has_numbers(), self.has_letters(), self.has_symbols()) {
            (true, true, true) => has_number && has_letter && has_symbol,
            (true, true, false) => has_number && has_letter && !has_symbol,
            (true, false, true) => has_number && !has_letter && has_symbol,
            (false, true, true) => !has_number && has_letter && has_symbol,
            (true, false, false) => has_number && !has_letter && !has_symbol,
            (false, true, false) => !has_number && has_letter && !has_symbol,
            (false, false, true) => !has_number && !has_letter && has_symbol,
            (false, false, false) => false,
        }
    }

    fn has_numbers(&self) -> bool {
        self.char_ranges.iter().any(|r| r.start >= 48 && r.end <= 58)
    }

    fn has_letters(&self) -> bool {
        self.char_ranges.iter().any(|r| (r.start >= 65 && r.end <= 91) || (r.start >= 97 && r.end <= 123))
    }

    fn has_symbols(&self) -> bool {
        self.char_ranges.iter().any(|r|
            (r.start >= 33 && r.end <= 48) ||
                (r.start >= 58 && r.end <= 65) ||
                (r.start >= 91 && r.end <= 97) ||
                (r.start >= 123 && r.end <= 127)
        )
    }
}
