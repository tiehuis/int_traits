//! Provides a single trait `IntTraits` which provides extended traits for the
//! integer type. These are typically special cases such as `sqrt` and are aimed
//! primarily at reducing the incessant casting that is otherwise required for
//! floored integer behaviour.

/// Provides functions which extended the class methods on integers.
pub trait IntTraits<T: Sized> where Self: Sized {
    /// Takes the floored square root of a number.
    ///
    /// ## Panics
    /// Panics if `n` is negative.
    fn sqrt(self) -> T;

    /// Takes the floored cubic root of a number.
    ///
    /// ## Panics
    /// Panics if `n` is negative.
    fn cbrt(self) -> T;

    /// Returns the floored logarithm of `n`.
    ///
    /// The logarithm must be of integer base. This is to avoid unnecessary
    /// casts and is purely ergonomic.
    ///
    /// ## Panics
    /// Panics if `n` <= 0.
    fn log(self, n: u64) -> T;

    /// Returns the floored base 10 logarithm of `n`.
    ///
    /// ## Panics
    /// Panics if `n` <= 0.
    fn log10(self) -> T {
        self.log(10 as u64)
    }

    /// Returns the floored base 10 logarithm of `n`.
    ///
    /// ## Panics
    /// Panics if `n` <= 0.
    fn log2(self) -> T {
        self.log(2 as u64)
    }
}

macro_rules! impl_int_trait {
    ($t:ty) => {
        impl IntTraits<$t> for $t {
            fn sqrt(self) -> $t {
                if self < 0 {
                    panic!("cannot take sqrt of a negative value: {}", self)
                }
                (self as f64).sqrt() as $t
            }

            fn cbrt(self) -> $t {
                if self < 0 {
                    panic!("cannot take cbrt of a negative value: {}", self)
                }
                (self as f64).cbrt() as $t
            }

            fn log(self, n: u64) -> $t {
                if self <= 0 {
                    panic!("cannot take log of a value less than or equal to 0: {}", self)
                }
                (self as f64).log(n as f64) as $t
            }
        }
    };
}

macro_rules! impl_uint_trait {
    ($t:ty) => {
        impl IntTraits<$t> for $t {
            fn sqrt(self) -> $t {
                (self as f64).sqrt() as $t
            }

            fn cbrt(self) -> $t {
                (self as f64).cbrt() as $t
            }

            fn log(self, n: u64) -> $t {
                if self == 0 {
                    panic!("cannot take log of a value less than or equal to 0: {}", self)
                }
                (self as f64).log(n as f64) as $t
            }
        }
    };
}

impl_int_trait!(i8);
impl_int_trait!(i16);
impl_int_trait!(i32);
impl_int_trait!(i64);
impl_int_trait!(isize);

impl_uint_trait!(u8);
impl_uint_trait!(u16);
impl_uint_trait!(u32);
impl_uint_trait!(u64);
impl_uint_trait!(usize);

#[cfg(test)]
mod tests {
    use super::IntTraits;

    #[test]
    fn unsigned_sqrt_overall() {
        assert_eq!(63_u8.sqrt(), 7);
        assert_eq!(63_u16.sqrt(), 7);
        assert_eq!(63_u32.sqrt(), 7);
        assert_eq!(63_u64.sqrt(), 7);
        assert_eq!(63_usize.sqrt(), 7);
    }

    #[test]
    fn signed_sqrt_overall() {
        assert_eq!(63_i8.sqrt(), 7);
        assert_eq!(63_i16.sqrt(), 7);
        assert_eq!(63_i32.sqrt(), 7);
        assert_eq!(63_i64.sqrt(), 7);
        assert_eq!(63_isize.sqrt(), 7);
    }

    #[test]
    fn unsigned_cbrt_overall() {
        assert_eq!(247_u8.cbrt(), 6);
        assert_eq!(891_u16.cbrt(), 9);
        assert_eq!(891_u32.cbrt(), 9);
        assert_eq!(891_u64.cbrt(), 9);
        assert_eq!(891_usize.cbrt(), 9);
    }

    #[test]
    fn signed_cbrt_overall() {
        assert_eq!(113_i8.cbrt(), 4);
        assert_eq!(891_i16.cbrt(), 9);
        assert_eq!(891_i32.cbrt(), 9);
        assert_eq!(891_i64.cbrt(), 9);
        assert_eq!(891_isize.cbrt(), 9);
    }

    #[test]
    #[should_panic]
    fn unsigned_zero_log() {
        let _ = 0.log(5);
    }

    #[test]
    #[should_panic]
    fn signed_zero_log() {
        let _ = 0_isize.log(5);
    }

    #[test]
    #[should_panic]
    fn signed_less_zero_log() {
        let _ = (-5).log(5);
    }

    #[test]
    fn zero_sqrt() {
        assert_eq!(0.sqrt(), 0);
    }

    #[test]
    fn zero_cbrt() {
        assert_eq!(0.cbrt(), 0);
    }
}
