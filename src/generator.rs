use rand::{thread_rng, Rng};
use std::char::from_u32;
use crate::config::PasswordConfig;

pub fn generate_password(config: &PasswordConfig) -> String {
    let mut rng = thread_rng();
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 1000;

    while attempts < MAX_ATTEMPTS {
        let mut result = String::with_capacity(config.length as usize);

        while result.len() < config.length as usize {
            let range_idx = rng.gen_range(0..config.char_ranges.len());
            let range = &config.char_ranges[range_idx];

            if let Some(c) = from_u32(rng.gen_range(range.clone())) {
                result.push(c);
            }
        }

        if config.validate_password(&result) {
            return result;
        }

        attempts += 1;
    }

    panic!("Failed to generate valid password after {} attempts", MAX_ATTEMPTS);
}
