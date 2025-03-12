use crate::KValue;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops,
};

/// The Number type used by the Koto runtime
///
/// The number can be either an `f32` or an `i32` depending on usage.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum KNumber {
    Float(f32),
    Int(i32),
}

impl KNumber {
    /// Returns NAN for the selected float size
    pub fn nan() -> Self {
        Self::Float(f32::NAN)
    }

    /// Returns the absolute value of the number
    #[must_use]
    pub fn abs(self) -> Self {
        match self {
            Self::Float(n) => Self::Float(n.abs()),
            Self::Int(n) => Self::Int(n.abs()),
        }
    }

    /// Returns the smallest integer greater than or equal to the number
    #[must_use]
    pub fn ceil(self) -> Self {
        match self {
            Self::Float(n) => Self::Int(n.ceil() as i32),
            Self::Int(n) => Self::Int(n),
        }
    }

    /// Returns the largest integer less than or equal to the number
    #[must_use]
    pub fn floor(self) -> Self {
        match self {
            Self::Float(n) => Self::Int(n.floor() as i32),
            Self::Int(n) => Self::Int(n),
        }
    }

    /// Returns the integer closest to the number
    ///
    /// Half-way values get rounded away from zero.
    #[must_use]
    pub fn round(self) -> Self {
        match self {
            Self::Float(n) => Self::Int(n.round() as i32),
            Self::Int(n) => Self::Int(n),
        }
    }

    /// Returns true if the number is represented by an `f32`
    pub fn is_float(self) -> bool {
        self.is_f32()
    }

    /// Returns true if the number is represented by an `f32`
    pub fn is_f32(self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns true if the number is represented by an `i32`
    pub fn is_int(self) -> bool {
        self.is_i32()
    }

    /// Returns true if the number is represented by an `i32`
    pub fn is_i32(self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Returns true if the integer version of the number is representable by an `f32`
    pub fn is_int_in_float_range(&self) -> bool {
        self.is_i32_in_f32_range()
    }

    /// Returns true if the integer version of the number is representable by an `f32`
    pub fn is_i32_in_f32_range(&self) -> bool {
        if let Self::Int(n) = *self {
            (n as f32 as i32) == n
        } else {
            false
        }
    }

    /// Returns true if the number is not NaN or infinity
    pub fn is_finite(self) -> bool {
        match self {
            Self::Float(n) => n.is_finite(),
            Self::Int(_) => true,
        }
    }

    /// Returns true if the number is NaN
    pub fn is_nan(self) -> bool {
        match self {
            Self::Float(n) => n.is_nan(),
            Self::Int(_) => false,
        }
    }

    /// Returns the result of raising self to the power of `other`
    ///
    /// If both inputs are i32s then the result will also be an i32,
    /// otherwise the result will be an f32.
    #[must_use]
    pub fn pow(self, other: Self) -> Self {
        use KNumber::*;

        match (self, other) {
            (Float(a), Float(b)) => Float(a.powf(b)),
            (Float(a), Int(b)) => Float(a.powf(b as f32)),
            (Int(a), Float(b)) => Float((a as f32).powf(b)),
            (Int(a), Int(b)) => Int(a.pow(b as u32)),
        }
    }

    /// Returns the value transmuted to a `u32`
    pub fn to_bits(self) -> u32 {
        match self {
            Self::Float(n) => n.to_bits(),
            Self::Int(n) => n as u32,
        }
    }
}

impl fmt::Debug for KNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KNumber::Float(n) => write!(f, "Float({n})"),
            KNumber::Int(n) => write!(f, "Int({n})"),
        }
    }
}

impl fmt::Display for KNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KNumber::Float(n) => {
                // Ensure that floats without fractional parts are rendered with a `.0` suffix
                if n.fract() == 0.0 {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            KNumber::Int(n) => write!(f, "{n}"),
        }
    }
}

impl Hash for KNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.to_bits())
    }
}

impl PartialEq for KNumber {
    fn eq(&self, other: &Self) -> bool {
        use KNumber::*;

        match (self, other) {
            (Float(a), Float(b)) => a == b,
            (Float(a), Int(b)) => *a == *b as f32,
            (Int(a), Float(b)) => *a as f32 == *b,
            (Int(a), Int(b)) => a == b,
        }
    }
}

impl Eq for KNumber {}

impl PartialOrd for KNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        use KNumber::*;

        let result = match (self, other) {
            (Float(a), Float(b)) => a.partial_cmp(b),
            (Float(a), Int(b)) => a.partial_cmp(&(*b as f32)),
            (Int(a), Float(b)) => (*a as f32).partial_cmp(b),
            (Int(a), Int(b)) => a.partial_cmp(b),
        };

        match result {
            Some(result) => result,
            None => match (self.is_nan(), other.is_nan()) {
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        }
    }
}

impl ops::Neg for KNumber {
    type Output = KNumber;

    fn neg(self) -> KNumber {
        use KNumber::*;

        match self {
            Float(n) => Float(-n),
            Int(n) => Int(-n),
        }
    }
}

impl ops::Neg for &KNumber {
    type Output = KNumber;

    fn neg(self) -> KNumber {
        use KNumber::*;

        match *self {
            Float(n) => Float(-n),
            Int(n) => Int(-n),
        }
    }
}

macro_rules! impl_from_knumber_ref {
    ($($type:ty),+) => {
        $(
            impl From<&KNumber> for $type {
                fn from(n: &KNumber) -> $type {
                    <$type>::from(*n)
                }
            }
        )+
    };
}

#[cfg(feature = "num32")]
macro_rules! impl_from_number_extras {
    ($($type:ty),+) => {
        $(
            impl From<&$type> for KNumber {
                fn from(n: &$type) -> KNumber {
                    Self::from(*n)
                }
            }

            impl From<$type> for KValue {
                fn from(value: $type) -> Self {
                    Self::Number(value.into())
                }
            }

            impl From<&$type> for KValue {
                fn from(value: &$type) -> Self {
                    Self::from(*value)
                }
            }
        )+
    };
}

macro_rules! number_traits_float {
    ($($type:ty),+) => {
        $(
            impl From<$type> for KNumber {
                fn from(n: $type) -> KNumber {
                    KNumber::Float(n as f32)
                }
            }
            impl_from_knumber_ref!($type);

            impl From<KNumber> for $type {
                fn from(n: KNumber) -> $type {
                    match n {
                        KNumber::Float(f) => f as $type,
                        KNumber::Int(i) => i as $type,
                    }
                }
            }
            impl_from_number_extras!($type);

            impl PartialEq<$type> for KNumber {
                fn eq(&self, b: &$type) -> bool {
                    let b = *b as f32;
                    match self {
                        KNumber::Float(a) => *a == b,
                        KNumber::Int(a) => *a as f32 == b,
                    }
                }
            }

            impl PartialOrd<$type> for KNumber {
                fn partial_cmp(&self, b: &$type) -> Option<Ordering> {
                    let b = *b as f32;
                    match self {
                        KNumber::Float(a) => a.partial_cmp(&b),
                        KNumber::Int(a) => (*a as f32).partial_cmp(&b),
                    }
                }
            }
        )+
    };
}

macro_rules! number_traits_int {
    ($($type:ty),+) => {
        $(
            impl From<$type> for KNumber {
                fn from(n: $type) -> KNumber {
                    use saturating_cast::SaturatingCast;
                    KNumber::Int(n.saturating_cast())
                }
            }
            impl_from_knumber_ref!($type);

            impl From<KNumber> for $type {
                fn from(n: KNumber) -> $type {
                    use saturating_cast::SaturatingCast;
                    match n {
                        KNumber::Float(f) => f as $type,
                        KNumber::Int(i) => i.saturating_cast(),
                    }
                }
            }
            impl_from_number_extras!($type);

            impl PartialEq<$type> for KNumber {
                fn eq(&self, b: &$type) -> bool {
                    let b = *b as i32;
                    match self {
                        KNumber::Float(a) => (*a as i32) == b,
                        KNumber::Int(a) => *a == b,
                    }
                }
            }

            impl PartialOrd<$type> for KNumber {
                fn partial_cmp(&self, b: &$type) -> Option<Ordering> {
                    let b = *b as i32;
                    match self {
                        KNumber::Float(a) => (*a as i32).partial_cmp(&b),
                        KNumber::Int(a) => a.partial_cmp(&b),
                    }
                }
            }
        )+
    };
}

number_traits_float!(f32, f64);
number_traits_int!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

macro_rules! number_op {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl ops::$trait for KNumber {
            type Output = KNumber;

            fn $fn(self, other: KNumber) -> KNumber {
                use KNumber::*;

                match (self, other) {
                    (Float(a), Float(b)) => Float(a $op b),
                    (Float(a), Int(b)) => Float(a $op b as f32),
                    (Int(a), Float(b)) => Float(a as f32 $op b),
                    (Int(a), Int(b)) => Int(a $op b),
                }
            }
        }

        impl ops::$trait for &KNumber {
            type Output = KNumber;

            fn $fn(self, other: &KNumber) -> KNumber {
                use KNumber::*;

                match (*self, *other) {
                    (Float(a), Float(b)) => Float(a $op b),
                    (Float(a), Int(b)) => Float(a $op b as f32),
                    (Int(a), Float(b)) => Float(a as f32 $op b),
                    (Int(a), Int(b)) => Int(a $op b),
                }
            }
        }
    };
}

number_op!(Add, add, +);
number_op!(Sub, sub, -);
number_op!(Mul, mul, *);
number_op!(Rem, rem, %);

impl ops::Div for KNumber {
    type Output = KNumber;

    fn div(self, other: KNumber) -> KNumber {
        use KNumber::*;

        match (self, other) {
            (Float(a), Float(b)) => Float(a / b),
            (Float(a), Int(b)) => Float(a / b as f32),
            (Int(a), Float(b)) => Float(a as f32 / b),
            (Int(a), Int(b)) => Float(a as f32 / b as f32),
        }
    }
}

impl ops::Div for &KNumber {
    type Output = KNumber;

    fn div(self, other: &KNumber) -> KNumber {
        use KNumber::*;

        match (*self, *other) {
            (Float(a), Float(b)) => Float(a / b),
            (Float(a), Int(b)) => Float(a / b as f32),
            (Int(a), Float(b)) => Float(a as f32 / b),
            (Int(a), Int(b)) => Float(a as f32 / b as f32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_out_of_range_saturates() {
        assert_eq!(u8::from(KNumber::from(1000)), u8::MAX);
        assert_eq!(i8::from(KNumber::from(-1000)), i8::MIN);
    }
}
