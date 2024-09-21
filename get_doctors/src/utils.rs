pub fn calculate_verification_digit(rut: u64) -> char {
    let mut sum = 0;
    let mut multiplier = 2;
    let mut rut_copy = rut;

    while rut_copy > 0 {
        sum += (rut_copy % 10) * multiplier;
        rut_copy /= 10;
        multiplier += 1;
        if multiplier > 7 {
            multiplier = 2;
        }
    }

    let digit = 11 - (sum % 11);
    match digit {
        11 => '0',
        10 => 'K',
        _ => std::char::from_digit(digit as u32, 10).unwrap(),
    }
}

pub fn format_rut_with_verification_digit(rut: u64) -> String {
    let verification_digit = calculate_verification_digit(rut);
    format!("{}-{}", rut, verification_digit)
}
