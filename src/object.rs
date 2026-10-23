use crate::number;

#[derive(Debug, Clone, Copy)]
pub struct NumberObject {
    pub value: i64,
}

impl NumberObject {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn is_even(&self) -> bool {
        number::is_even(self.value)
    }

    pub fn is_odd(&self) -> bool {
        number::is_odd(self.value)
    }

    pub fn is_prime(&self) -> bool {
        number::is_prime(self.value)
    }

    pub fn sentence(&self) -> String {
        number::description(self.value)
    }
}
