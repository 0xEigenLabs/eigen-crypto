use std::prelude::v1::*;
use std::hash::*;

fn hash<T: Hash>(x: &T) -> u64 {
    use std::collections::hash_map::RandomState;
    let mut hasher = <RandomState as BuildHasher>::Hasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

//#[cfg(feature = "bigint")]
use num_rational::BigRational;
use num_rational::{Ratio, Rational, Rational64};

use core::f64;
use core::i32;
use core::str::FromStr;
use integer::Integer;
use traits::{FromPrimitive, One, Pow, Signed, Zero};

pub const _0: Rational = Ratio { numer: 0, denom: 1 };
pub const _1: Rational = Ratio { numer: 1, denom: 1 };
pub const _2: Rational = Ratio { numer: 2, denom: 1 };
pub const _NEG2: Rational = Ratio {
    numer: -2,
    denom: 1,
};
pub const _1_2: Rational = Ratio { numer: 1, denom: 2 };
pub const _3_2: Rational = Ratio { numer: 3, denom: 2 };
pub const _NEG1_2: Rational = Ratio {
    numer: -1,
    denom: 2,
};
pub const _1_NEG2: Rational = Ratio {
    numer: 1,
    denom: -2,
};
pub const _NEG1_NEG2: Rational = Ratio {
    numer: -1,
    denom: -2,
};
pub const _1_3: Rational = Ratio { numer: 1, denom: 3 };
pub const _NEG1_3: Rational = Ratio {
    numer: -1,
    denom: 3,
};
pub const _2_3: Rational = Ratio { numer: 2, denom: 3 };
pub const _NEG2_3: Rational = Ratio {
    numer: -2,
    denom: 3,
};

//#[cfg(feature = "bigint")]
fn to_big(n: Rational) -> BigRational {
    Ratio::new(
        FromPrimitive::from_isize(n.numer).unwrap(),
        FromPrimitive::from_isize(n.denom).unwrap(),
    )
}
//#[cfg(not(feature = "bigint"))]
//pub fn to_big(n: Rational) -> Rational {
//    Ratio::new(
//        FromPrimitive::from_isize(n.numer).unwrap(),
//        FromPrimitive::from_isize(n.denom).unwrap(),
//    )
//}

//#[test]
pub fn test_test_constants() {
    // check our constants are what Ratio::new etc. would make.
    assert_eq!(_0, Zero::zero());
    assert_eq!(_1, One::one());
    assert_eq!(_2, Ratio::from_integer(2));
    assert_eq!(_1_2, Ratio::new(1, 2));
    assert_eq!(_3_2, Ratio::new(3, 2));
    assert_eq!(_NEG1_2, Ratio::new(-1, 2));
    assert_eq!(_2, From::from(2));
}

//#[test]
pub fn test_new_reduce() {
    let one22 = Ratio::new(2, 2);

    assert_eq!(one22, One::one());
}
#[test]
#[should_panic]
fn test_new_zero() {
    let _a = Ratio::new(1, 0);
}

//#[test]
pub fn test_approximate_float() {
    assert_eq!(Ratio::from_f32(0.5f32), Some(Ratio::new(1i64, 2)));
    assert_eq!(Ratio::from_f64(0.5f64), Some(Ratio::new(1i32, 2)));
    assert_eq!(Ratio::from_f32(5f32), Some(Ratio::new(5i64, 1)));
    assert_eq!(Ratio::from_f64(5f64), Some(Ratio::new(5i32, 1)));
    assert_eq!(Ratio::from_f32(29.97f32), Some(Ratio::new(2997i64, 100)));
    assert_eq!(Ratio::from_f32(-29.97f32), Some(Ratio::new(-2997i64, 100)));

    assert_eq!(Ratio::<i8>::from_f32(63.5f32), Some(Ratio::new(127i8, 2)));
    assert_eq!(Ratio::<i8>::from_f32(126.5f32), Some(Ratio::new(126i8, 1)));
    assert_eq!(Ratio::<i8>::from_f32(127.0f32), Some(Ratio::new(127i8, 1)));
    assert_eq!(Ratio::<i8>::from_f32(127.5f32), None);
    assert_eq!(Ratio::<i8>::from_f32(-63.5f32), Some(Ratio::new(-127i8, 2)));
    assert_eq!(
        Ratio::<i8>::from_f32(-126.5f32),
        Some(Ratio::new(-126i8, 1))
    );
    assert_eq!(
        Ratio::<i8>::from_f32(-127.0f32),
        Some(Ratio::new(-127i8, 1))
    );
    assert_eq!(Ratio::<i8>::from_f32(-127.5f32), None);

    assert_eq!(Ratio::<u8>::from_f32(-127f32), None);
    assert_eq!(Ratio::<u8>::from_f32(127f32), Some(Ratio::new(127u8, 1)));
    assert_eq!(Ratio::<u8>::from_f32(127.5f32), Some(Ratio::new(255u8, 2)));
    assert_eq!(Ratio::<u8>::from_f32(256f32), None);

    assert_eq!(Ratio::<i64>::from_f64(-10e200), None);
    assert_eq!(Ratio::<i64>::from_f64(10e200), None);
    assert_eq!(Ratio::<i64>::from_f64(f64::INFINITY), None);
    assert_eq!(Ratio::<i64>::from_f64(f64::NEG_INFINITY), None);
    assert_eq!(Ratio::<i64>::from_f64(f64::NAN), None);
    assert_eq!(
        Ratio::<i64>::from_f64(f64::EPSILON),
        Some(Ratio::new(1, 4503599627370496))
    );
    assert_eq!(Ratio::<i64>::from_f64(0.0), Some(Ratio::new(0, 1)));
    assert_eq!(Ratio::<i64>::from_f64(-0.0), Some(Ratio::new(0, 1)));
}

//#[test]
pub fn test_cmp() {
    assert!(_0 == _0 && _1 == _1);
    assert!(_0 != _1 && _1 != _0);
    assert!(_0 < _1 && !(_1 < _0));
    assert!(_1 > _0 && !(_0 > _1));

    assert!(_0 <= _0 && _1 <= _1);
    assert!(_0 <= _1 && !(_1 <= _0));

    assert!(_0 >= _0 && _1 >= _1);
    assert!(_1 >= _0 && !(_0 >= _1));
}

//#[test]
pub fn test_cmp_overflow() {
    use core::cmp::Ordering;

    // issue #7 example:
    let big = Ratio::new(128u8, 1);
    let small = big.recip();
    assert!(big > small);

    // try a few that are closer together
    // (some matching numer, some matching denom, some neither)
    let ratios = [
        Ratio::new(125_i8, 127_i8),
        Ratio::new(63_i8, 64_i8),
        Ratio::new(124_i8, 125_i8),
        Ratio::new(125_i8, 126_i8),
        Ratio::new(126_i8, 127_i8),
        Ratio::new(127_i8, 126_i8),
    ];

    fn check_cmp(a: Ratio<i8>, b: Ratio<i8>, ord: Ordering) {
        //#[cfg(feature = "std")]
        println!("comparing {} and {}", a, b);
        assert_eq!(a.cmp(&b), ord);
        assert_eq!(b.cmp(&a), ord.reverse());
    }

    for (i, &a) in ratios.iter().enumerate() {
        check_cmp(a, a, Ordering::Equal);
        check_cmp(-a, a, Ordering::Less);
        for &b in &ratios[i + 1..] {
            check_cmp(a, b, Ordering::Less);
            check_cmp(-a, -b, Ordering::Greater);
            check_cmp(a.recip(), b.recip(), Ordering::Greater);
            check_cmp(-a.recip(), -b.recip(), Ordering::Less);
        }
    }
}

//#[test]
pub fn test_to_integer() {
    assert_eq!(_0.to_integer(), 0);
    assert_eq!(_1.to_integer(), 1);
    assert_eq!(_2.to_integer(), 2);
    assert_eq!(_1_2.to_integer(), 0);
    assert_eq!(_3_2.to_integer(), 1);
    assert_eq!(_NEG1_2.to_integer(), 0);
}

//#[test]
pub fn test_numer() {
    assert_eq!(_0.numer(), &0);
    assert_eq!(_1.numer(), &1);
    assert_eq!(_2.numer(), &2);
    assert_eq!(_1_2.numer(), &1);
    assert_eq!(_3_2.numer(), &3);
    assert_eq!(_NEG1_2.numer(), &(-1));
}
//#[test]
pub fn test_denom() {
    assert_eq!(_0.denom(), &1);
    assert_eq!(_1.denom(), &1);
    assert_eq!(_2.denom(), &1);
    assert_eq!(_1_2.denom(), &2);
    assert_eq!(_3_2.denom(), &2);
    assert_eq!(_NEG1_2.denom(), &2);
}

//#[test]
pub fn test_is_integer() {
    assert!(_0.is_integer());
    assert!(_1.is_integer());
    assert!(_2.is_integer());
    assert!(!_1_2.is_integer());
    assert!(!_3_2.is_integer());
    assert!(!_NEG1_2.is_integer());
}

//#[test]
//#[cfg(feature = "std")]
pub fn test_show() {
    assert_eq!(format!("{}", _2), "2".to_string());
    assert_eq!(format!("{}", _1_2), "1/2".to_string());
    assert_eq!(format!("{}", _0), "0".to_string());
    assert_eq!(format!("{}", Ratio::from_integer(-2)), "-2".to_string());
}

pub mod arith {
    use num_rational::{Ratio, Rational};
    use super::{to_big, _0, _1, _1_2, _2, _3_2, _NEG1_2};
    use traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

    //#[test]
    pub fn test_add() {
        fn test(a: Rational, b: Rational, c: Rational) {
            assert_eq!(a + b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x += b;
                    x
                },
                c
            );
            assert_eq!(to_big(a) + to_big(b), to_big(c));
            assert_eq!(a.checked_add(&b), Some(c));
            assert_eq!(to_big(a).checked_add(&to_big(b)), Some(to_big(c)));
        }
        fn test_assign(a: Rational, b: isize, c: Rational) {
            assert_eq!(a + b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x += b;
                    x
                },
                c
            );
        }

        test(_1, _1_2, _3_2);
        test(_1, _1, _2);
        test(_1_2, _3_2, _2);
        test(_1_2, _NEG1_2, _0);
        test_assign(_1_2, 1, _3_2);
    }

    //#[test]
    pub fn test_sub() {
        fn test(a: Rational, b: Rational, c: Rational) {
            assert_eq!(a - b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x -= b;
                    x
                },
                c
            );
            assert_eq!(to_big(a) - to_big(b), to_big(c));
            assert_eq!(a.checked_sub(&b), Some(c));
            assert_eq!(to_big(a).checked_sub(&to_big(b)), Some(to_big(c)));
        }
        fn test_assign(a: Rational, b: isize, c: Rational) {
            assert_eq!(a - b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x -= b;
                    x
                },
                c
            );
        }

        test(_1, _1_2, _1_2);
        test(_3_2, _1_2, _1);
        test(_1, _NEG1_2, _3_2);
        test_assign(_1_2, 1, _NEG1_2);
    }

    //#[test]
    pub fn test_mul() {
        fn test(a: Rational, b: Rational, c: Rational) {
            assert_eq!(a * b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x *= b;
                    x
                },
                c
            );
            assert_eq!(to_big(a) * to_big(b), to_big(c));
            assert_eq!(a.checked_mul(&b), Some(c));
            assert_eq!(to_big(a).checked_mul(&to_big(b)), Some(to_big(c)));
        }
        fn test_assign(a: Rational, b: isize, c: Rational) {
            assert_eq!(a * b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x *= b;
                    x
                },
                c
            );
        }

        test(_1, _1_2, _1_2);
        test(_1_2, _3_2, Ratio::new(3, 4));
        test(_1_2, _NEG1_2, Ratio::new(-1, 4));
        test_assign(_1_2, 2, _1);
    }

    //#[test]
    pub fn test_div() {
        fn test(a: Rational, b: Rational, c: Rational) {
            assert_eq!(a / b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x /= b;
                    x
                },
                c
            );
            assert_eq!(to_big(a) / to_big(b), to_big(c));
            assert_eq!(a.checked_div(&b), Some(c));
            assert_eq!(to_big(a).checked_div(&to_big(b)), Some(to_big(c)));
        }
        fn test_assign(a: Rational, b: isize, c: Rational) {
            assert_eq!(a / b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x /= b;
                    x
                },
                c
            );
        }

        test(_1, _1_2, _2);
        test(_3_2, _1_2, _1 + _2);
        test(_1, _NEG1_2, _NEG1_2 + _NEG1_2 + _NEG1_2 + _NEG1_2);
        test_assign(_1, 2, _1_2);
    }

    //#[test]
    pub fn test_rem() {
        fn test(a: Rational, b: Rational, c: Rational) {
            assert_eq!(a % b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x %= b;
                    x
                },
                c
            );
            assert_eq!(to_big(a) % to_big(b), to_big(c))
        }
        fn test_assign(a: Rational, b: isize, c: Rational) {
            assert_eq!(a % b, c);
            assert_eq!(
                {
                    let mut x = a;
                    x %= b;
                    x
                },
                c
            );
        }

        test(_3_2, _1, _1_2);
        test(_2, _NEG1_2, _0);
        test(_1_2, _2, _1_2);
        test_assign(_3_2, 1, _1_2);
    }

    //#[test]
    pub fn test_neg() {
        fn test(a: Rational, b: Rational) {
            assert_eq!(-a, b);
            assert_eq!(-to_big(a), to_big(b))
        }

        test(_0, _0);
        test(_1_2, _NEG1_2);
        test(-_1, _1);
    }
    //#[test]
    pub fn test_zero() {
        assert_eq!(_0 + _0, _0);
        assert_eq!(_0 * _0, _0);
        assert_eq!(_0 * _1, _0);
        assert_eq!(_0 / _NEG1_2, _0);
        assert_eq!(_0 - _0, _0);
    }
    //#[test]
    //#[should_panic]
    pub fn test_div_0() {
        let _a = _1 / _0;
    }

    //#[test]
    pub fn test_checked_failures() {
        let big = Ratio::new(128u8, 1);
        let small = Ratio::new(1, 128u8);
        assert_eq!(big.checked_add(&big), None);
        assert_eq!(small.checked_sub(&big), None);
        assert_eq!(big.checked_mul(&big), None);
        assert_eq!(small.checked_div(&big), None);
        assert_eq!(_1.checked_div(&_0), None);
    }
}

//#[test]
pub fn test_round() {
    assert_eq!(_1_3.ceil(), _1);
    assert_eq!(_1_3.floor(), _0);
    assert_eq!(_1_3.round(), _0);
    assert_eq!(_1_3.trunc(), _0);

    assert_eq!(_NEG1_3.ceil(), _0);
    assert_eq!(_NEG1_3.floor(), -_1);
    assert_eq!(_NEG1_3.round(), _0);
    assert_eq!(_NEG1_3.trunc(), _0);

    assert_eq!(_2_3.ceil(), _1);
    assert_eq!(_2_3.floor(), _0);
    assert_eq!(_2_3.round(), _1);
    assert_eq!(_2_3.trunc(), _0);

    assert_eq!(_NEG2_3.ceil(), _0);
    assert_eq!(_NEG2_3.floor(), -_1);
    assert_eq!(_NEG2_3.round(), -_1);
    assert_eq!(_NEG2_3.trunc(), _0);

    assert_eq!(_1_2.ceil(), _1);
    assert_eq!(_1_2.floor(), _0);
    assert_eq!(_1_2.round(), _1);
    assert_eq!(_1_2.trunc(), _0);

    assert_eq!(_NEG1_2.ceil(), _0);
    assert_eq!(_NEG1_2.floor(), -_1);
    assert_eq!(_NEG1_2.round(), -_1);
    assert_eq!(_NEG1_2.trunc(), _0);

    assert_eq!(_1.ceil(), _1);
    assert_eq!(_1.floor(), _1);
    assert_eq!(_1.round(), _1);
    assert_eq!(_1.trunc(), _1);

    // Overflow checks

    let _neg1 = Ratio::from_integer(-1);
    let _large_rat1 = Ratio::new(i32::MAX, i32::MAX - 1);
    let _large_rat2 = Ratio::new(i32::MAX - 1, i32::MAX);
    let _large_rat3 = Ratio::new(i32::MIN + 2, i32::MIN + 1);
    let _large_rat4 = Ratio::new(i32::MIN + 1, i32::MIN + 2);
    let _large_rat5 = Ratio::new(i32::MIN + 2, i32::MAX);
    let _large_rat6 = Ratio::new(i32::MAX, i32::MIN + 2);
    let _large_rat7 = Ratio::new(1, i32::MIN + 1);
    let _large_rat8 = Ratio::new(1, i32::MAX);

    assert_eq!(_large_rat1.round(), One::one());
    assert_eq!(_large_rat2.round(), One::one());
    assert_eq!(_large_rat3.round(), One::one());
    assert_eq!(_large_rat4.round(), One::one());
    assert_eq!(_large_rat5.round(), _neg1);
    assert_eq!(_large_rat6.round(), _neg1);
    assert_eq!(_large_rat7.round(), Zero::zero());
    assert_eq!(_large_rat8.round(), Zero::zero());
}

//#[test]
pub fn test_fract() {
    assert_eq!(_1.fract(), _0);
    assert_eq!(_NEG1_2.fract(), _NEG1_2);
    assert_eq!(_1_2.fract(), _1_2);
    assert_eq!(_3_2.fract(), _1_2);
}

//#[test]
pub fn test_recip() {
    assert_eq!(_1 * _1.recip(), _1);
    assert_eq!(_2 * _2.recip(), _1);
    assert_eq!(_1_2 * _1_2.recip(), _1);
    assert_eq!(_3_2 * _3_2.recip(), _1);
    assert_eq!(_NEG1_2 * _NEG1_2.recip(), _1);

    assert_eq!(_3_2.recip(), _2_3);
    assert_eq!(_NEG1_2.recip(), _NEG2);
    assert_eq!(_NEG1_2.recip().denom(), &1);
}

#[test]
#[should_panic(expected = "== 0")]
fn test_recip_fail() {
    let _a = Ratio::new(0, 1).recip();
}

//#[test]
pub fn test_pow() {
    fn test(r: Rational, e: i32, expected: Rational) {
        assert_eq!(r.pow(e), expected);
        assert_eq!(Pow::pow(r, e), expected);
        assert_eq!(Pow::pow(r, &e), expected);
        assert_eq!(Pow::pow(&r, e), expected);
        assert_eq!(Pow::pow(&r, &e), expected);
    }

    test(_1_2, 2, Ratio::new(1, 4));
    test(_1_2, -2, Ratio::new(4, 1));
    test(_1, 1, _1);
    test(_1, i32::MAX, _1);
    test(_1, i32::MIN, _1);
    test(_NEG1_2, 2, _1_2.pow(2i32));
    test(_NEG1_2, 3, -_1_2.pow(3i32));
    test(_3_2, 0, _1);
    test(_3_2, -1, _3_2.recip());
    test(_3_2, 3, Ratio::new(27, 8));
}

//#[test]
//#[cfg(feature = "std")]
pub fn test_to_from_str() {
    //use std::string::{String, ToString};
    fn test(r: Rational, s: String) {
        assert_eq!(FromStr::from_str(&s), Ok(r));
        assert_eq!(r.to_string(), s);
    }
    test(_1, "1".to_string());
    test(_0, "0".to_string());
    test(_1_2, "1/2".to_string());
    test(_3_2, "3/2".to_string());
    test(_2, "2".to_string());
    test(_NEG1_2, "-1/2".to_string());
}
//#[test]
pub fn test_from_str_fail() {
    fn test(s: &str) {
        let rational: Result<Rational, _> = FromStr::from_str(s);
        assert!(rational.is_err());
    }

    let xs = ["0 /1", "abc", "", "1/", "--1/2", "3/2/1", "1/0"];
    for &s in xs.iter() {
        test(s);
    }
}

//#[cfg(feature = "bigint")]
//#[test]
pub fn test_from_float() {
    use traits::float::FloatCore;
    fn test<T: FloatCore>(given: T, (numer, denom): (&str, &str)) {
        let ratio: BigRational = Ratio::from_float(given).unwrap();
        assert_eq!(
            ratio,
            Ratio::new(
                FromStr::from_str(numer).unwrap(),
                FromStr::from_str(denom).unwrap()
            )
        );
    }

    // f32
    test(3.14159265359f32, ("13176795", "4194304"));
    test(2f32.powf(100.), ("1267650600228229401496703205376", "1"));
    test(-2f32.powf(100.), ("-1267650600228229401496703205376", "1"));
    test(
        1.0 / 2f32.powf(100.),
        ("1", "1267650600228229401496703205376"),
    );
    test(684729.48391f32, ("1369459", "2"));
    test(-8573.5918555f32, ("-4389679", "512"));

    // f64
    test(3.14159265359f64, ("3537118876014453", "1125899906842624"));
    test(2f64.powf(100.), ("1267650600228229401496703205376", "1"));
    test(-2f64.powf(100.), ("-1267650600228229401496703205376", "1"));
    test(684729.48391f64, ("367611342500051", "536870912"));
    test(-8573.5918555f64, ("-4713381968463931", "549755813888"));
    test(
        1.0 / 2f64.powf(100.),
        ("1", "1267650600228229401496703205376"),
    );
}

//#[cfg(feature = "bigint")]
//#[test]
pub fn test_from_float_fail() {
    use core::{f32};

    assert_eq!(Ratio::from_float(f32::NAN), None);
    assert_eq!(Ratio::from_float(f32::INFINITY), None);
    assert_eq!(Ratio::from_float(f32::NEG_INFINITY), None);
    assert_eq!(Ratio::from_float(f64::NAN), None);
    assert_eq!(Ratio::from_float(f64::INFINITY), None);
    assert_eq!(Ratio::from_float(f64::NEG_INFINITY), None);
}

//#[test]
pub fn test_signed() {
    assert_eq!(_NEG1_2.abs(), _1_2);
    assert_eq!(_3_2.abs_sub(&_1_2), _1);
    assert_eq!(_1_2.abs_sub(&_3_2), Zero::zero());
    assert_eq!(_1_2.signum(), One::one());
    assert_eq!(_NEG1_2.signum(), -<Ratio<isize>>::one());
    assert_eq!(_0.signum(), Zero::zero());
    assert!(_NEG1_2.is_negative());
    assert!(_1_NEG2.is_negative());
    assert!(!_NEG1_2.is_positive());
    assert!(!_1_NEG2.is_positive());
    assert!(_1_2.is_positive());
    assert!(_NEG1_NEG2.is_positive());
    assert!(!_1_2.is_negative());
    assert!(!_NEG1_NEG2.is_negative());
    assert!(!_0.is_positive());
    assert!(!_0.is_negative());
}

//#[test]
//#[cfg(feature = "std")]
pub fn test_hash() {
    assert!(hash(&_0) != hash(&_1));
    assert!(hash(&_0) != hash(&_3_2));

    // a == b -> hash(a) == hash(b)
    let a = Rational::new_raw(4, 2);
    let b = Rational::new_raw(6, 3);
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));

    let a = Rational::new_raw(123456789, 1000);
    let b = Rational::new_raw(123456789 * 5, 5000);
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
}

//#[test]
pub fn test_into_pair() {
    assert_eq!((0, 1), _0.into());
    assert_eq!((-2, 1), _NEG2.into());
    assert_eq!((1, -2), _1_NEG2.into());
}

//#[test]
pub fn test_from_pair() {
    assert_eq!(_0, Ratio::from((0, 1)));
    assert_eq!(_1, Ratio::from((1, 1)));
    assert_eq!(_NEG2, Ratio::from((-2, 1)));
    assert_eq!(_1_NEG2, Ratio::from((1, -2)));
}

//#[test]
pub fn ratio_iter_sum() {
    // generic function to assure the iter method can be called
    // for any Iterator with Item = Ratio<impl Integer> or Ratio<&impl Integer>
    fn iter_sums<T: Integer + Clone>(slice: &[Ratio<T>]) -> [Ratio<T>; 3] {
        let mut manual_sum = Ratio::new(T::zero(), T::one());
        for ratio in slice {
            manual_sum = manual_sum + ratio;
        }
        [manual_sum, slice.iter().sum(), slice.iter().cloned().sum()]
    }
    // collect into array so test works on no_std
    let mut nums = [Ratio::new(0, 1); 1000];
    for (i, r) in (0..1000).map(|n| Ratio::new(n, 500)).enumerate() {
        nums[i] = r;
    }
    let sums = iter_sums(&nums[..]);
    assert_eq!(sums[0], sums[1]);
    assert_eq!(sums[0], sums[2]);
}

//#[test]
pub fn ratio_iter_product() {
    // generic function to assure the iter method can be called
    // for any Iterator with Item = Ratio<impl Integer> or Ratio<&impl Integer>
    fn iter_products<T: Integer + Clone>(slice: &[Ratio<T>]) -> [Ratio<T>; 3] {
        let mut manual_prod = Ratio::new(T::one(), T::one());
        for ratio in slice {
            manual_prod = manual_prod * ratio;
        }
        [
            manual_prod,
            slice.iter().product(),
            slice.iter().cloned().product(),
        ]
    }

    // collect into array so test works on no_std
    let mut nums = [Ratio::new(0, 1); 1000];
    for (i, r) in (0..1000).map(|n| Ratio::new(n, 500)).enumerate() {
        nums[i] = r;
    }
    let products = iter_products(&nums[..]);
    assert_eq!(products[0], products[1]);
    assert_eq!(products[0], products[2]);
}

//#[test]
pub fn test_num_zero() {
    let zero = Rational64::zero();
    assert!(zero.is_zero());

    let mut r = Rational64::new(123, 456);
    assert!(!r.is_zero());
    assert_eq!(&r + &zero, r);

    r.set_zero();
    assert!(r.is_zero());
}

//#[test]
pub fn test_num_one() {
    let one = Rational64::one();
    assert!(one.is_one());

    let mut r = Rational64::new(123, 456);
    assert!(!r.is_one());
    assert_eq!(&r * &one, r);

    r.set_one();
    assert!(r.is_one());
}
