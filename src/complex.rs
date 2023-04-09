use std::cmp::max;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
    pub exponent: u64,
}

impl Complex {
    pub fn new(new_real: f64, new_imagnary: f64, new_exponent: u64) -> Self {
        Self {
            real: new_real,
            imaginary: new_imagnary,
            exponent: new_exponent,
        }
    }

    pub fn complex_abs(complex_number: Complex) -> f64 {
        let result = complex_number.real.powf(2.0) + complex_number.imaginary.powf(2.0);
        result.sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
            exponent: max(self.exponent, other.exponent),
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
            exponent: max(self.exponent, other.exponent),
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
            exponent: max(self.exponent, other.exponent),
        }
    }
}
