use password_generator::{PasswordConfig, generate_password};

#[test]
fn test_password_length() {
    let config = PasswordConfig::new(24, true, true, true);
    let password = generate_password(&config);
    assert_eq!(password.len(), 24);
}

#[test]
fn test_numbers_only() {
    let config = PasswordConfig::new(10, true, false, false);
    let password = generate_password(&config);
    assert!(password.chars().all(|c| c.is_numeric()));
}

#[test]
fn test_letters_only() {
    let config = PasswordConfig::new(10, false, true, false);
    let password = generate_password(&config);
    assert!(password.chars().all(|c| c.is_alphabetic()));
}

#[test]
fn test_symbols_only() {
    let config = PasswordConfig::new(10, false, false, true);
    let password = generate_password(&config);
    assert!(password.chars().all(|c| !c.is_alphanumeric()));
}

#[test]
fn test_all_character_types() {
    let config = PasswordConfig::new(24, true, true, true);
    let password = generate_password(&config);

    assert!(password.chars().any(|c| c.is_numeric()));
    assert!(password.chars().any(|c| c.is_alphabetic()));
    assert!(password.chars().any(|c| !c.is_alphanumeric()));
}
