#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    Negative,
    Zero,
    One,
    EvenPrime,
    EvenComposite,
    OddPrime,
    OddComposite,
}

pub fn is_even(value: i64) -> bool {
    value % 2 == 0
}

pub fn is_odd(value: i64) -> bool {
    value % 2 != 0
}

pub fn is_prime(value: i64) -> bool {
    if value < 2 {
        return false;
    }
    if value == 2 {
        return true;
    }
    if value % 2 == 0 {
        return false;
    }

    let mut divisor = 3_i64;
    while divisor <= value / divisor {
        if value % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

pub fn classify(value: i64) -> NumberKind {
    if value < 0 {
        NumberKind::Negative
    } else if value == 0 {
        NumberKind::Zero
    } else if value == 1 {
        NumberKind::One
    } else if is_prime(value) {
        NumberKind::EvenPrime
    } else if is_even(value) {
        NumberKind::EvenComposite
    } else {
        NumberKind::OddComposite
    }
}

pub fn description(value: i64) -> String {
    match classify(value) {
        NumberKind::Negative => format!("{value} is negative; it is neither prime nor composite."),
        NumberKind::Zero => "0 is even and is neither prime nor composite.".to_string(),
        NumberKind::One => "1 is odd and is neither prime nor composite.".to_string(),
        NumberKind::EvenPrime => format!("{value} is even and prime; 2 is the only even prime."),
        NumberKind::EvenComposite => format!("{value} is even and composite because it has divisors other than 1 and itself."),
        NumberKind::OddPrime => format!("{value} is odd and prime."),
        NumberKind::OddComposite => format!("{value} is odd and composite."),
    }
}
