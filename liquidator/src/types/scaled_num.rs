use std::{
    cmp::{max, min, Ordering},
    fmt::{self, Display, Formatter},
    ops::{Add, Mul},
};

use ethers::types::U256;

// TODO: impl +=, /, and from u64
#[derive(Clone, Copy)]
pub struct ScaledNum {
    pub num: U256,
    pub scale: u8,
}

impl ScaledNum {
    pub fn new<U: Into<U256>>(num: U, scale: u8) -> Self {
        Self {
            num: num.into(),
            scale,
        }
    }

    pub fn zero() -> Self {
        Self {
            num: U256::zero(),
            scale: 0,
        }
    }
}

impl Mul for ScaledNum {
    type Output = ScaledNum;

    // is it always best to take the higher scale?
    // pro: never lose precision
    // con: could eventually overflow with too many operation
    fn mul(self, other: ScaledNum) -> ScaledNum {
        let min_scale = min(self.scale, other.scale);
        let max_scale = max(self.scale, other.scale);
        let num = self.num * other.num / U256::exp10(min_scale as usize);
        ScaledNum {
            num,
            scale: max_scale,
        }
    }
}

impl PartialEq for ScaledNum {
    fn eq(&self, other: &Self) -> bool {
        // both are exactly equal
        if self.scale == other.scale && self.num == other.num {
            true
        // they are clearly not equal (one has a larger scale and smaller num)
        } else if (self.scale > other.scale && self.num < other.num)
            || self.scale < other.scale && self.num > other.num
        {
            false
        } else {
            // if we get here then one has a larger (or eq) scale and num

            // find which one has the larger num and scale
            let (larger, smaller) = if self.scale > other.scale {
                (self, other)
            } else {
                (other, self)
            };

            // get the smaller num into same units as the larger num and compare
            smaller.num * U256::exp10((larger.scale - smaller.scale).into()) == larger.num
        }
    }
}

impl Eq for ScaledNum {}

// TODO: what's clippy on about here?
impl PartialOrd for ScaledNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.scale > other.scale {
            // case where self has the larger scale, get into units of self
            if other.num * U256::exp10((self.scale - other.scale).into()) < self.num {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else if self.scale < other.scale {
            // case where other has the larger scale, get into units of other
            if self.num * U256::exp10((other.scale - self.scale).into()) > other.num {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else {
            // same scale, compare nums
            if self.num > other.num {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl Ord for ScaledNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for ScaledNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.scale == 0 {
            return write!(f, "{}", self.num);
        }

        let mut num_str = self.num.to_string();
        let scale = self.scale as usize;
        if scale + 1 > num_str.len() {
            let desired_length = num_str.len() + (scale - num_str.len()) + 1;
            num_str = format!("{:0>width$}", num_str, width = desired_length);
        }
        let decimal_position = num_str.len() - scale;
        let num = &num_str.to_string()[..(decimal_position)];
        let mantissa = &num_str.to_string()[(decimal_position)..];
        write!(f, "{}.{}", num, mantissa)
    }
}

impl Add for ScaledNum {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let max_scale = max(self.scale, other.scale);
        let scale_diff_self = max_scale - self.scale;
        let scale_diff_other = max_scale - other.scale;

        let adjusted_self = self.num * U256::exp10(scale_diff_self as usize);
        let adjusted_other = other.num * U256::exp10(scale_diff_other as usize);

        ScaledNum {
            num: adjusted_self + adjusted_other,
            scale: max_scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_scaled_num() {
        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x * y;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 2000.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(100, 2);
        let z = x * y;

        assert_eq!(z.scale, 2);
        assert_eq!(z.num, 100.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x * y * x;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 2000.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = ScaledNum::new(300, 4);
        let q = x * y * z;

        assert_eq!(q.scale, 4);
        assert_eq!(q.num, 600.into());
    }

    #[test]
    fn test_scale_of_zero() {
        let x = ScaledNum::new(100, 0);
        let y = ScaledNum::new(2000, 3);
        let z = x * y;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 200000.into());

        let x = ScaledNum::new(1, 0);
        let y = ScaledNum::new(2, 0);
        let z = x * y;

        assert_eq!(z.scale, 0);
        assert_eq!(z.num, 2.into());
    }

    #[test]
    fn test_scaled_num_as_mantissa() {
        let x = ScaledNum::new(50, 2);
        let y = ScaledNum::new(10000, 1);
        let z = x * y;

        assert_eq!(z.scale, 2);

        assert_eq!(z.num, 50000.into());
    }

    #[test]
    fn test_scaled_num_eq_and_cmp() {
        let x = ScaledNum::new(1, 1);
        let y = ScaledNum::new(1, 2);
        assert!(x != y);
        assert!(x > y);

        let x = ScaledNum::new(1000, 1);
        let y = ScaledNum::new(1000, 1);
        assert!(x == y);

        let x = ScaledNum::new(1000, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x == y);

        let x = ScaledNum::new(2000, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);
        assert!(x > y);

        let x = ScaledNum::new(2000, 1);
        let y = ScaledNum::new(100, 2);
        assert!(x != y);
        assert!(x > y);

        let x = ScaledNum::new(2000, 1);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);
        assert!(x > y);

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);
        assert!(y > x);
    }

    #[test]
    fn test_display() {
        let x = ScaledNum::new(10, 1);
        assert_eq!(format!("{x}"), format!("1.0"));

        let x = ScaledNum::new(11, 1);
        assert_eq!(format!("{x}"), format!("1.1"));

        let x = ScaledNum::new(11, 3);
        assert_eq!(format!("{x}"), format!("0.011"));

        let x = ScaledNum::new(11, 2);
        assert_eq!(format!("{x}"), format!("0.11"));

        let x = ScaledNum::new(1, 0);
        assert_eq!(format!("{x}"), format!("1"));

        let x = ScaledNum::new(999962450000000000_u64, 18);
        assert_eq!(format!("{x}"), format!("0.999962450000000000"));
    }

    #[test]
    fn test_addition() {
        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x + y;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 3000.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(100, 2);
        let z = x + y;

        assert_eq!(z.scale, 2);
        assert_eq!(z.num, 200.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x + y + x;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 4000.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = ScaledNum::new(300, 4); // 0.03
        let q = x + y + z;

        assert_eq!(q.scale, 4);
        assert_eq!(q.num, 30300.into());
    }
}
