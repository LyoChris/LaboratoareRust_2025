use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::{fmt, fmt::Debug, fmt::Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

//Am implementat in plus AddAssign, SubAssign, MulAssign, Div, DivAssign si Default

impl Complex {
    fn new<T1, T2>(r: T1, i: T2) -> Self
    where
        f64: From<T1>,
        f64: From<T2>,
    {
        Complex {
            real: f64::from(r),
            imag: f64::from(i),
        }
    }

    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -(self.imag),
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex {
            real: 0.0,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(r: f64) -> Self {
        Complex { real: r, imag: 0.0 }
    }
}

impl From<i32> for Complex {
    fn from(r: i32) -> Self {
        Complex {
            real: r as f64,
            imag: 0.0,
        }
    }
}

impl<T> Add<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn add(self, other: T) -> Self::Output {
        let oth = Complex::from(other);
        Complex {
            real: self.real + oth.real,
            imag: self.imag + oth.imag,
        }
    }
}

impl<T> Sub<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn sub(self, other: T) -> Self::Output {
        let oth = Complex::from(other);
        Complex {
            real: self.real - oth.real,
            imag: self.imag - oth.imag,
        }
    }
}

impl<T> Mul<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn mul(self, other: T) -> Self::Output {
        let oth = Complex::from(other);
        let new_real = (self.real * oth.real) - (self.imag * oth.imag);
        let new_imag = (self.real * oth.imag) + (self.imag * oth.real);
        Complex {
            real: new_real,
            imag: new_imag,
        }
    }
}

impl<T> Div<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn div(self, rhs: T) -> Self::Output {
        let oth = Complex::from(rhs);
        let new_real = ((self.real * oth.real) + (self.imag * oth.imag))
            / ((oth.real * oth.real) + (oth.imag * oth.imag));
        let new_imag = ((self.imag * oth.real) - (self.real * oth.imag))
            / ((oth.real * oth.real) + (oth.imag * oth.imag));
        Complex {
            real: new_real,
            imag: new_imag,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    Complex: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        let oth = Complex::from(rhs);
        *self = Self {
            real: self.real + oth.real,
            imag: self.imag + oth.imag,
        };
    }
}

impl<T> SubAssign<T> for Complex
where
    Complex: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        let oth = Complex::from(rhs);
        *self = Self {
            real: self.real - oth.real,
            imag: self.imag - oth.imag,
        };
    }
}

impl<T> MulAssign<T> for Complex
where
    Complex: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        let oth = Complex::from(rhs);
        let new_real = (self.real * oth.real) - (self.imag * oth.imag);
        let new_img = (self.real * oth.imag) + (self.imag * oth.real);
        *self = Self::new(new_real, new_img);
    }
}

impl<T> DivAssign<T> for Complex
where
    Complex: From<T>,
{
    fn div_assign(&mut self, rhs: T) {
        let oth = Complex::from(rhs);
        let new_real = ((self.real * oth.real) + (self.imag * oth.imag))
            / ((oth.real * oth.real) + (oth.imag * oth.imag));
        let new_imag = ((self.imag * oth.real) - (self.real * oth.imag))
            / ((oth.real * oth.real) + (oth.imag * oth.imag));
        *self = Self {
            real: new_real,
            imag: new_imag,
        };
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Complex {
                real: 0.0,
                imag: 0.0,
            } => write!(f, "0"),
            Complex { real, imag: 0.0 } => write!(f, "{real}"),
            Complex { real: 0.0, imag } => write!(f, "{imag}i"),
            Complex { real, imag } if *imag > 1.00 => write!(f, "{real}+{imag}i"),
            Complex { real, imag } => write!(f, "{real}{imag}i"),
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    let mut a = Complex::new(1.0, 2.0);
    a += Complex::new(3.0, 4.0);
    assert_eq_rel!(a.real, 4.0);
    assert_eq_rel!(a.imag, 6.0);

    let mut b = Complex::new(-1.0, 5.0);
    b += 2.0;
    assert_eq_rel!(b.real, 1.0);
    assert_eq_rel!(b.imag, 5.0);

    let mut c = Complex::new(5.0, 7.0);
    c -= Complex::new(2.0, 3.0);
    assert_eq_rel!(c.real, 3.0);
    assert_eq_rel!(c.imag, 4.0);

    let mut d = Complex::new(4.0, -2.0);
    d -= 1.0;
    assert_eq_rel!(d.real, 3.0);
    assert_eq_rel!(d.imag, -2.0);

    let mut e = Complex::new(1.0, 2.0);
    e *= Complex::new(3.0, 4.0);
    assert_eq_rel!(e.real, -5.0);
    assert_eq_rel!(e.imag, 10.0);

    let mut f = Complex::new(2.0, 3.0);
    f *= 2.0;
    assert_eq_rel!(f.real, 4.0);
    assert_eq_rel!(f.imag, 6.0);

    let a = Complex::new(4.0, 2.0);
    let b = Complex::new(2.0, 1.0);
    let c = a / b;
    assert_eq_rel!(c.real, 2.0);
    assert_eq_rel!(c.imag, 0.0);

    let d = Complex::new(3.0, 4.0);
    let e = d / 2.0;
    assert_eq_rel!(e.real, 1.5);
    assert_eq_rel!(e.imag, 2.0);

    let mut f = Complex::new(4.0, 2.0);
    f /= Complex::new(2.0, 1.0);
    assert_eq_rel!(f.real, 2.0);
    assert_eq_rel!(f.imag, 0.0);

    let mut g = Complex::new(3.0, 4.0);
    g /= 2.0;
    assert_eq_rel!(g.real, 1.5);
    assert_eq_rel!(g.imag, 2.0);

    let c = Complex::default();
    assert_eq_rel!(c.real, 0.0);
    assert_eq_rel!(c.imag, 0.0);

    assert_eq!(c.to_string(), "0");

    println!("ok!");
}
