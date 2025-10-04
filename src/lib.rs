pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug)]
pub enum MatrixError {
    NotSquare,
    SingularMatrix,
}

// Fraction type for exact rational arithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }
        let mut frac = Fraction {
            numerator,
            denominator,
        };
        frac.simplify();
        frac
    }

    pub fn from_f64(value: f64) -> Self {
        // Convert float to fraction using continued fractions
        let sign = if value < 0.0 { -1 } else { 1 };
        let abs_val = value.abs();

        if (abs_val - abs_val.round()).abs() < 1e-10 {
            return Fraction::new(sign * (abs_val.round() as i64), 1);
        }

        let (num, den) = float_to_rational(abs_val, 1000000);
        Fraction::new(sign * num, den)
    }

    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    fn simplify(&mut self) {
        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        self.numerator /= gcd;
        self.denominator /= gcd;

        // Keep denominator positive
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }

    pub fn add(&self, other: &Fraction) -> Fraction {
        let num = self.numerator * other.denominator + other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den)
    }

    pub fn sub(&self, other: &Fraction) -> Fraction {
        let num = self.numerator * other.denominator - other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den)
    }

    pub fn mul(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn div(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }

    pub fn abs(&self) -> Fraction {
        Fraction::new(self.numerator.abs(), self.denominator)
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// GCD calculation
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Convert float to rational using continued fractions
fn float_to_rational(x: f64, max_denominator: i64) -> (i64, i64) {
    let mut h1 = 1i64;
    let mut h2 = 0i64;
    let mut k1 = 0i64;
    let mut k2 = 1i64;
    let mut b = x;

    loop {
        let a = b.floor() as i64;
        let h = a * h1 + h2;
        let k = a * k1 + k2;

        if k > max_denominator {
            return (h1, k1);
        }

        let diff = (x - (h as f64 / k as f64)).abs();
        if diff < 1e-10 {
            return (h, k);
        }

        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;

        if (b - a as f64).abs() < 1e-10 {
            return (h, k);
        }

        b = 1.0 / (b - a as f64);
    }
}

pub type FractionMatrix = Vec<Vec<Fraction>>;

pub fn matrix_to_fraction(matrix: &Matrix) -> FractionMatrix {
    matrix
        .iter()
        .map(|row| row.iter().map(|&val| Fraction::from_f64(val)).collect())
        .collect()
}

pub fn fraction_to_matrix(frac_matrix: &FractionMatrix) -> Matrix {
    frac_matrix
        .iter()
        .map(|row| row.iter().map(|frac| frac.to_f64()).collect())
        .collect()
}

pub fn print_matrix(matrix: &Matrix) {
    for row in matrix {
        print!("[");
        for (j, val) in row.iter().enumerate() {
            if j > 0 {
                print!(" ");
            }
            print!("{:10.4}", val);
        }
        println!(" ]");
    }
}

pub fn print_matrix_fraction(frac_matrix: &FractionMatrix) {
    // Calculate max width for each column
    let mut col_widths = vec![0; frac_matrix[0].len()];
    for row in frac_matrix {
        for (j, frac) in row.iter().enumerate() {
            let width = format!("{}", frac).len();
            if width > col_widths[j] {
                col_widths[j] = width;
            }
        }
    }

    for row in frac_matrix {
        print!("[");
        for (j, frac) in row.iter().enumerate() {
            if j > 0 {
                print!(" ");
            }
            print!("{:>width$}", frac, width = col_widths[j]);
        }
        println!(" ]");
    }
}
