use std::{
    cmp::{max, min, Ordering},
    ops::Mul,
};

use ethers::types::U256;

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

impl PartialOrd for ScaledNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            if self.scale > other.scale {
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
}

impl Ord for ScaledNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
}
