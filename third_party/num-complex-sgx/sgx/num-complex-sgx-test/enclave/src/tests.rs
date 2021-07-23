use std::hash;
//#[cfg(test)]
fn hash<T: hash::Hash>(x: &T) -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = <RandomState as BuildHasher>::Hasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

//#[cfg(test)]
pub mod test {
    #![allow(non_upper_case_globals)]

    use num_complex::{Complex, Complex64};
    use core::f64;
    use core::str::FromStr;
    use std::prelude::v1::*;

    use std::string::{String, ToString};

    use traits::{Num, One, Zero};

    pub const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    pub const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    pub const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    pub const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    pub const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    pub const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    pub const all_consts: [Complex64; 5] = [_0_0i, _1_0i, _1_1i, _neg1_1i, _05_05i];
    pub const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    //#[test]
    pub fn test_consts() {
        // check our constants are what Complex::new creates
        fn test(c: Complex64, r: f64, i: f64) {
            assert_eq!(c, Complex::new(r, i));
        }
        test(_0_0i, 0.0, 0.0);
        test(_1_0i, 1.0, 0.0);
        test(_1_1i, 1.0, 1.0);
        test(_neg1_1i, -1.0, 1.0);
        test(_05_05i, 0.5, 0.5);

        assert_eq!(_0_0i, Zero::zero());
        assert_eq!(_1_0i, One::one());
    }

    //#[test]
    pub fn test_scale_unscale() {
        assert_eq!(_05_05i.scale(2.0), _1_1i);
        assert_eq!(_1_1i.unscale(2.0), _05_05i);
        for &c in all_consts.iter() {
            assert_eq!(c.scale(2.0).unscale(2.0), c);
        }
    }

    //#[test]
    pub fn test_conj() {
        for &c in all_consts.iter() {
            assert_eq!(c.conj(), Complex::new(c.re, -c.im));
            assert_eq!(c.conj().conj(), c);
        }
    }

    //#[test]
    pub fn test_inv() {
        assert_eq!(_1_1i.inv(), _05_05i.conj());
        assert_eq!(_1_0i.inv(), _1_0i.inv());
    }

    //#[test]
    //#[should_panic]
    pub fn test_divide_by_zero_natural() {
        let n = Complex::new(2, 3);
        let d = Complex::new(0, 0);
        let _x = n / d;
    }

    //#[test]
    pub fn test_inv_zero() {
        // FIXME #20: should this really fail, or just NaN?
        assert!(_0_0i.inv().is_nan());
    }

    //#[test]
    pub fn test_l1_norm() {
        assert_eq!(_0_0i.l1_norm(), 0.0);
        assert_eq!(_1_0i.l1_norm(), 1.0);
        assert_eq!(_1_1i.l1_norm(), 2.0);
        assert_eq!(_0_1i.l1_norm(), 1.0);
        assert_eq!(_neg1_1i.l1_norm(), 2.0);
        assert_eq!(_05_05i.l1_norm(), 1.0);
        assert_eq!(_4_2i.l1_norm(), 6.0);
    }

    //#[test]
    pub fn test_pow() {
        for c in all_consts.iter() {
            assert_eq!(c.powi(0), _1_0i);
            let mut pos = _1_0i;
            let mut neg = _1_0i;
            for i in 1i32..20 {
                pos *= c;
                assert_eq!(pos, c.powi(i));
                if c.is_zero() {
                    assert!(c.powi(-i).is_nan());
                } else {
                    neg /= c;
                    assert_eq!(neg, c.powi(-i));
                }
            }
        }
    }

    //#[cfg(feature = "std")]
    pub mod float {
        use super::*;
        use traits::{Float, Pow};

        //#[test]
        //#[cfg_attr(target_arch = "x86", ignore)]
        // FIXME #7158: (maybe?) currently failing on x86.
        pub fn test_norm() {
            fn test(c: Complex64, ns: f64) {
                assert_eq!(c.norm_sqr(), ns);
                assert_eq!(c.norm(), ns.sqrt())
            }
            test(_0_0i, 0.0);
            test(_1_0i, 1.0);
            test(_1_1i, 2.0);
            test(_neg1_1i, 2.0);
            test(_05_05i, 0.5);
        }

        //#[test]
        pub fn test_arg() {
            fn test(c: Complex64, arg: f64) {
                assert!((c.arg() - arg).abs() < 1.0e-6)
            }
            test(_1_0i, 0.0);
            test(_1_1i, 0.25 * f64::consts::PI);
            test(_neg1_1i, 0.75 * f64::consts::PI);
            test(_05_05i, 0.25 * f64::consts::PI);
        }

        //#[test]
        pub fn test_polar_conv() {
            fn test(c: Complex64) {
                let (r, theta) = c.to_polar();
                assert!((c - Complex::from_polar(&r, &theta)).norm() < 1e-6);
            }
            for &c in all_consts.iter() {
                test(c);
            }
        }

        fn close(a: Complex64, b: Complex64) -> bool {
            close_to_tol(a, b, 1e-10)
        }

        fn close_to_tol(a: Complex64, b: Complex64, tol: f64) -> bool {
            // returns true if a and b are reasonably close
            let close = (a == b) || (a - b).norm() < tol;
            if !close {
                println!("{:?} != {:?}", a, b);
            }
            close
        }

        //#[test]
        pub fn test_exp() {
            assert!(close(_1_0i.exp(), _1_0i.scale(f64::consts::E)));
            assert!(close(_0_0i.exp(), _1_0i));
            assert!(close(_0_1i.exp(), Complex::new(1.0.cos(), 1.0.sin())));
            assert!(close(_05_05i.exp() * _05_05i.exp(), _1_1i.exp()));
            assert!(close(
                _0_1i.scale(-f64::consts::PI).exp(),
                _1_0i.scale(-1.0)
            ));
            for &c in all_consts.iter() {
                // e^conj(z) = conj(e^z)
                assert!(close(c.conj().exp(), c.exp().conj()));
                // e^(z + 2 pi i) = e^z
                assert!(close(
                    c.exp(),
                    (c + _0_1i.scale(f64::consts::PI * 2.0)).exp()
                ));
            }
        }

        //#[test]
        pub fn test_ln() {
            assert!(close(_1_0i.ln(), _0_0i));
            assert!(close(_0_1i.ln(), _0_1i.scale(f64::consts::PI / 2.0)));
            assert!(close(_0_0i.ln(), Complex::new(f64::neg_infinity(), 0.0)));
            assert!(close(
                (_neg1_1i * _05_05i).ln(),
                _neg1_1i.ln() + _05_05i.ln()
            ));
            for &c in all_consts.iter() {
                // ln(conj(z() = conj(ln(z))
                assert!(close(c.conj().ln(), c.ln().conj()));
                // for this branch, -pi <= arg(ln(z)) <= pi
                assert!(-f64::consts::PI <= c.ln().arg() && c.ln().arg() <= f64::consts::PI);
            }
        }

        //#[test]
        pub fn test_powc() {
            let a = Complex::new(2.0, -3.0);
            let b = Complex::new(3.0, 0.0);
            assert!(close(a.powc(b), a.powf(b.re)));
            assert!(close(b.powc(a), a.expf(b.re)));
            let c = Complex::new(1.0 / 3.0, 0.1);
            assert!(close_to_tol(
                a.powc(c),
                Complex::new(1.65826, -0.33502),
                1e-5
            ));
        }

        //#[test]
        pub fn test_powf() {
            let c = Complex64::new(2.0, -1.0);
            let expected = Complex64::new(-0.8684746, -16.695934);
            assert!(close_to_tol(c.powf(3.5), expected, 1e-5));
            assert!(close_to_tol(Pow::pow(c, 3.5_f64), expected, 1e-5));
            assert!(close_to_tol(Pow::pow(c, 3.5_f32), expected, 1e-5));
        }

        //#[test]
        pub fn test_log() {
            let c = Complex::new(2.0, -1.0);
            let r = c.log(10.0);
            assert!(close_to_tol(r, Complex::new(0.349485, -0.20135958), 1e-5));
        }

        //#[test]
        pub fn test_some_expf_cases() {
            let c = Complex::new(2.0, -1.0);
            let r = c.expf(10.0);
            assert!(close_to_tol(r, Complex::new(-66.82015, -74.39803), 1e-5));

            let c = Complex::new(5.0, -2.0);
            let r = c.expf(3.4);
            assert!(close_to_tol(r, Complex::new(-349.25, -290.63), 1e-2));

            let c = Complex::new(-1.5, 2.0 / 3.0);
            let r = c.expf(1.0 / 3.0);
            assert!(close_to_tol(r, Complex::new(3.8637, -3.4745), 1e-2));
        }

        //#[test]
        pub fn test_sqrt() {
            assert!(close(_0_0i.sqrt(), _0_0i));
            assert!(close(_1_0i.sqrt(), _1_0i));
            assert!(close(Complex::new(-1.0, 0.0).sqrt(), _0_1i));
            assert!(close(Complex::new(-1.0, -0.0).sqrt(), _0_1i.scale(-1.0)));
            assert!(close(_0_1i.sqrt(), _05_05i.scale(2.0.sqrt())));
            for &c in all_consts.iter() {
                // sqrt(conj(z() = conj(sqrt(z))
                assert!(close(c.conj().sqrt(), c.sqrt().conj()));
                // for this branch, -pi/2 <= arg(sqrt(z)) <= pi/2
                assert!(
                    -f64::consts::PI / 2.0 <= c.sqrt().arg()
                        && c.sqrt().arg() <= f64::consts::PI / 2.0
                );
                // sqrt(z) * sqrt(z) = z
                assert!(close(c.sqrt() * c.sqrt(), c));
            }
        }

        //#[test]
        pub fn test_sin() {
            assert!(close(_0_0i.sin(), _0_0i));
            assert!(close(_1_0i.scale(f64::consts::PI * 2.0).sin(), _0_0i));
            assert!(close(_0_1i.sin(), _0_1i.scale(1.0.sinh())));
            for &c in all_consts.iter() {
                // sin(conj(z)) = conj(sin(z))
                assert!(close(c.conj().sin(), c.sin().conj()));
                // sin(-z) = -sin(z)
                assert!(close(c.scale(-1.0).sin(), c.sin().scale(-1.0)));
            }
        }

        //#[test]
        pub fn test_cos() {
            assert!(close(_0_0i.cos(), _1_0i));
            assert!(close(_1_0i.scale(f64::consts::PI * 2.0).cos(), _1_0i));
            assert!(close(_0_1i.cos(), _1_0i.scale(1.0.cosh())));
            for &c in all_consts.iter() {
                // cos(conj(z)) = conj(cos(z))
                assert!(close(c.conj().cos(), c.cos().conj()));
                // cos(-z) = cos(z)
                assert!(close(c.scale(-1.0).cos(), c.cos()));
            }
        }

        //#[test]
        pub fn test_tan() {
            assert!(close(_0_0i.tan(), _0_0i));
            assert!(close(_1_0i.scale(f64::consts::PI / 4.0).tan(), _1_0i));
            assert!(close(_1_0i.scale(f64::consts::PI).tan(), _0_0i));
            for &c in all_consts.iter() {
                // tan(conj(z)) = conj(tan(z))
                assert!(close(c.conj().tan(), c.tan().conj()));
                // tan(-z) = -tan(z)
                assert!(close(c.scale(-1.0).tan(), c.tan().scale(-1.0)));
            }
        }

        //#[test]
        pub fn test_asin() {
            assert!(close(_0_0i.asin(), _0_0i));
            assert!(close(_1_0i.asin(), _1_0i.scale(f64::consts::PI / 2.0)));
            assert!(close(
                _1_0i.scale(-1.0).asin(),
                _1_0i.scale(-f64::consts::PI / 2.0)
            ));
            assert!(close(_0_1i.asin(), _0_1i.scale((1.0 + 2.0.sqrt()).ln())));
            for &c in all_consts.iter() {
                // asin(conj(z)) = conj(asin(z))
                assert!(close(c.conj().asin(), c.asin().conj()));
                // asin(-z) = -asin(z)
                assert!(close(c.scale(-1.0).asin(), c.asin().scale(-1.0)));
                // for this branch, -pi/2 <= asin(z).re <= pi/2
                assert!(
                    -f64::consts::PI / 2.0 <= c.asin().re && c.asin().re <= f64::consts::PI / 2.0
                );
            }
        }

        //#[test]
        pub fn test_acos() {
            assert!(close(_0_0i.acos(), _1_0i.scale(f64::consts::PI / 2.0)));
            assert!(close(_1_0i.acos(), _0_0i));
            assert!(close(
                _1_0i.scale(-1.0).acos(),
                _1_0i.scale(f64::consts::PI)
            ));
            assert!(close(
                _0_1i.acos(),
                Complex::new(f64::consts::PI / 2.0, (2.0.sqrt() - 1.0).ln())
            ));
            for &c in all_consts.iter() {
                // acos(conj(z)) = conj(acos(z))
                assert!(close(c.conj().acos(), c.acos().conj()));
                // for this branch, 0 <= acos(z).re <= pi
                assert!(0.0 <= c.acos().re && c.acos().re <= f64::consts::PI);
            }
        }

        //#[test]
        pub fn test_atan() {
            assert!(close(_0_0i.atan(), _0_0i));
            assert!(close(_1_0i.atan(), _1_0i.scale(f64::consts::PI / 4.0)));
            assert!(close(
                _1_0i.scale(-1.0).atan(),
                _1_0i.scale(-f64::consts::PI / 4.0)
            ));
            assert!(close(_0_1i.atan(), Complex::new(0.0, f64::infinity())));
            for &c in all_consts.iter() {
                // atan(conj(z)) = conj(atan(z))
                assert!(close(c.conj().atan(), c.atan().conj()));
                // atan(-z) = -atan(z)
                assert!(close(c.scale(-1.0).atan(), c.atan().scale(-1.0)));
                // for this branch, -pi/2 <= atan(z).re <= pi/2
                assert!(
                    -f64::consts::PI / 2.0 <= c.atan().re && c.atan().re <= f64::consts::PI / 2.0
                );
            }
        }

        //#[test]
        pub fn test_sinh() {
            assert!(close(_0_0i.sinh(), _0_0i));
            assert!(close(
                _1_0i.sinh(),
                _1_0i.scale((f64::consts::E - 1.0 / f64::consts::E) / 2.0)
            ));
            assert!(close(_0_1i.sinh(), _0_1i.scale(1.0.sin())));
            for &c in all_consts.iter() {
                // sinh(conj(z)) = conj(sinh(z))
                assert!(close(c.conj().sinh(), c.sinh().conj()));
                // sinh(-z) = -sinh(z)
                assert!(close(c.scale(-1.0).sinh(), c.sinh().scale(-1.0)));
            }
        }

        //#[test]
        pub fn test_cosh() {
            assert!(close(_0_0i.cosh(), _1_0i));
            assert!(close(
                _1_0i.cosh(),
                _1_0i.scale((f64::consts::E + 1.0 / f64::consts::E) / 2.0)
            ));
            assert!(close(_0_1i.cosh(), _1_0i.scale(1.0.cos())));
            for &c in all_consts.iter() {
                // cosh(conj(z)) = conj(cosh(z))
                assert!(close(c.conj().cosh(), c.cosh().conj()));
                // cosh(-z) = cosh(z)
                assert!(close(c.scale(-1.0).cosh(), c.cosh()));
            }
        }

        //#[test]
        pub fn test_tanh() {
            assert!(close(_0_0i.tanh(), _0_0i));
            assert!(close(
                _1_0i.tanh(),
                _1_0i.scale((f64::consts::E.powi(2) - 1.0) / (f64::consts::E.powi(2) + 1.0))
            ));
            assert!(close(_0_1i.tanh(), _0_1i.scale(1.0.tan())));
            for &c in all_consts.iter() {
                // tanh(conj(z)) = conj(tanh(z))
                assert!(close(c.conj().tanh(), c.conj().tanh()));
                // tanh(-z) = -tanh(z)
                assert!(close(c.scale(-1.0).tanh(), c.tanh().scale(-1.0)));
            }
        }

        //#[test]
        pub fn test_asinh() {
            assert!(close(_0_0i.asinh(), _0_0i));
            assert!(close(_1_0i.asinh(), _1_0i.scale(1.0 + 2.0.sqrt()).ln()));
            assert!(close(_0_1i.asinh(), _0_1i.scale(f64::consts::PI / 2.0)));
            assert!(close(
                _0_1i.asinh().scale(-1.0),
                _0_1i.scale(-f64::consts::PI / 2.0)
            ));
            for &c in all_consts.iter() {
                // asinh(conj(z)) = conj(asinh(z))
                assert!(close(c.conj().asinh(), c.conj().asinh()));
                // asinh(-z) = -asinh(z)
                assert!(close(c.scale(-1.0).asinh(), c.asinh().scale(-1.0)));
                // for this branch, -pi/2 <= asinh(z).im <= pi/2
                assert!(
                    -f64::consts::PI / 2.0 <= c.asinh().im && c.asinh().im <= f64::consts::PI / 2.0
                );
            }
        }

        //#[test]
        pub fn test_acosh() {
            assert!(close(_0_0i.acosh(), _0_1i.scale(f64::consts::PI / 2.0)));
            assert!(close(_1_0i.acosh(), _0_0i));
            assert!(close(
                _1_0i.scale(-1.0).acosh(),
                _0_1i.scale(f64::consts::PI)
            ));
            for &c in all_consts.iter() {
                // acosh(conj(z)) = conj(acosh(z))
                assert!(close(c.conj().acosh(), c.conj().acosh()));
                // for this branch, -pi <= acosh(z).im <= pi and 0 <= acosh(z).re
                assert!(
                    -f64::consts::PI <= c.acosh().im
                        && c.acosh().im <= f64::consts::PI
                        && 0.0 <= c.cosh().re
                );
            }
        }

        //#[test]
        pub fn test_atanh() {
            assert!(close(_0_0i.atanh(), _0_0i));
            assert!(close(_0_1i.atanh(), _0_1i.scale(f64::consts::PI / 4.0)));
            assert!(close(_1_0i.atanh(), Complex::new(f64::infinity(), 0.0)));
            for &c in all_consts.iter() {
                // atanh(conj(z)) = conj(atanh(z))
                assert!(close(c.conj().atanh(), c.conj().atanh()));
                // atanh(-z) = -atanh(z)
                assert!(close(c.scale(-1.0).atanh(), c.atanh().scale(-1.0)));
                // for this branch, -pi/2 <= atanh(z).im <= pi/2
                assert!(
                    -f64::consts::PI / 2.0 <= c.atanh().im && c.atanh().im <= f64::consts::PI / 2.0
                );
            }
        }

        //#[test]
        pub fn test_exp_ln() {
            for &c in all_consts.iter() {
                // e^ln(z) = z
                assert!(close(c.ln().exp(), c));
            }
        }

        //#[test]
        pub fn test_trig_to_hyperbolic() {
            for &c in all_consts.iter() {
                // sin(iz) = i sinh(z)
                assert!(close((_0_1i * c).sin(), _0_1i * c.sinh()));
                // cos(iz) = cosh(z)
                assert!(close((_0_1i * c).cos(), c.cosh()));
                // tan(iz) = i tanh(z)
                assert!(close((_0_1i * c).tan(), _0_1i * c.tanh()));
            }
        }

        //#[test]
        pub fn test_trig_identities() {
            for &c in all_consts.iter() {
                // tan(z) = sin(z)/cos(z)
                assert!(close(c.tan(), c.sin() / c.cos()));
                // sin(z)^2 + cos(z)^2 = 1
                assert!(close(c.sin() * c.sin() + c.cos() * c.cos(), _1_0i));

                // sin(asin(z)) = z
                assert!(close(c.asin().sin(), c));
                // cos(acos(z)) = z
                assert!(close(c.acos().cos(), c));
                // tan(atan(z)) = z
                // i and -i are branch points
                if c != _0_1i && c != _0_1i.scale(-1.0) {
                    assert!(close(c.atan().tan(), c));
                }

                // sin(z) = (e^(iz) - e^(-iz))/(2i)
                assert!(close(
                    ((_0_1i * c).exp() - (_0_1i * c).exp().inv()) / _0_1i.scale(2.0),
                    c.sin()
                ));
                // cos(z) = (e^(iz) + e^(-iz))/2
                assert!(close(
                    ((_0_1i * c).exp() + (_0_1i * c).exp().inv()).unscale(2.0),
                    c.cos()
                ));
                // tan(z) = i (1 - e^(2iz))/(1 + e^(2iz))
                assert!(close(
                    _0_1i * (_1_0i - (_0_1i * c).scale(2.0).exp())
                        / (_1_0i + (_0_1i * c).scale(2.0).exp()),
                    c.tan()
                ));
            }
        }

        //#[test]
        pub fn test_hyperbolic_identites() {
            for &c in all_consts.iter() {
                // tanh(z) = sinh(z)/cosh(z)
                assert!(close(c.tanh(), c.sinh() / c.cosh()));
                // cosh(z)^2 - sinh(z)^2 = 1
                assert!(close(c.cosh() * c.cosh() - c.sinh() * c.sinh(), _1_0i));

                // sinh(asinh(z)) = z
                assert!(close(c.asinh().sinh(), c));
                // cosh(acosh(z)) = z
                assert!(close(c.acosh().cosh(), c));
                // tanh(atanh(z)) = z
                // 1 and -1 are branch points
                if c != _1_0i && c != _1_0i.scale(-1.0) {
                    assert!(close(c.atanh().tanh(), c));
                }

                // sinh(z) = (e^z - e^(-z))/2
                assert!(close((c.exp() - c.exp().inv()).unscale(2.0), c.sinh()));
                // cosh(z) = (e^z + e^(-z))/2
                assert!(close((c.exp() + c.exp().inv()).unscale(2.0), c.cosh()));
                // tanh(z) = ( e^(2z) - 1)/(e^(2z) + 1)
                assert!(close(
                    (c.scale(2.0).exp() - _1_0i) / (c.scale(2.0).exp() + _1_0i),
                    c.tanh()
                ));
            }
        }
    }

    // Test both a + b and a += b
    macro_rules! test_a_op_b {
        ($a:ident + $b:expr, $answer:expr) => {
            assert_eq!($a + $b, $answer);
            assert_eq!(
                {
                    let mut x = $a;
                    x += $b;
                    x
                },
                $answer
            );
        };
        ($a:ident - $b:expr, $answer:expr) => {
            assert_eq!($a - $b, $answer);
            assert_eq!(
                {
                    let mut x = $a;
                    x -= $b;
                    x
                },
                $answer
            );
        };
        ($a:ident * $b:expr, $answer:expr) => {
            assert_eq!($a * $b, $answer);
            assert_eq!(
                {
                    let mut x = $a;
                    x *= $b;
                    x
                },
                $answer
            );
        };
        ($a:ident / $b:expr, $answer:expr) => {
            assert_eq!($a / $b, $answer);
            assert_eq!(
                {
                    let mut x = $a;
                    x /= $b;
                    x
                },
                $answer
            );
        };
        ($a:ident % $b:expr, $answer:expr) => {
            assert_eq!($a % $b, $answer);
            assert_eq!(
                {
                    let mut x = $a;
                    x %= $b;
                    x
                },
                $answer
            );
        };
    }

    // Test both a + b and a + &b
    macro_rules! test_op {
        ($a:ident $op:tt $b:expr, $answer:expr) => {
            test_a_op_b!($a $op $b, $answer);
            test_a_op_b!($a $op &$b, $answer);
        };
    }

    pub mod complex_arithmetic {
        use super::{_05_05i, _0_0i, _0_1i, _1_0i, _1_1i, _4_2i, _neg1_1i, all_consts};
        use traits::{MulAdd, MulAddAssign, Zero};

        //#[test]
        pub fn test_add() {
            test_op!(_05_05i + _05_05i, _1_1i);
            test_op!(_0_1i + _1_0i, _1_1i);
            test_op!(_1_0i + _neg1_1i, _0_1i);

            for &c in all_consts.iter() {
                test_op!(_0_0i + c, c);
                test_op!(c + _0_0i, c);
            }
        }

        //#[test]
        pub fn test_sub() {
            test_op!(_05_05i - _05_05i, _0_0i);
            test_op!(_0_1i - _1_0i, _neg1_1i);
            test_op!(_0_1i - _neg1_1i, _1_0i);

            for &c in all_consts.iter() {
                test_op!(c - _0_0i, c);
                test_op!(c - c, _0_0i);
            }
        }

        //#[test]
        pub fn test_mul() {
            test_op!(_05_05i * _05_05i, _0_1i.unscale(2.0));
            test_op!(_1_1i * _0_1i, _neg1_1i);

            // i^2 & i^4
            test_op!(_0_1i * _0_1i, -_1_0i);
            assert_eq!(_0_1i * _0_1i * _0_1i * _0_1i, _1_0i);

            for &c in all_consts.iter() {
                test_op!(c * _1_0i, c);
                test_op!(_1_0i * c, c);
            }
        }

        //#[test]
        //#[cfg(feature = "std")]
        pub fn test_mul_add_float() {
            assert_eq!(_05_05i.mul_add(_05_05i, _0_0i), _05_05i * _05_05i + _0_0i);
            assert_eq!(_05_05i * _05_05i + _0_0i, _05_05i.mul_add(_05_05i, _0_0i));
            assert_eq!(_0_1i.mul_add(_0_1i, _0_1i), _neg1_1i);
            assert_eq!(_1_0i.mul_add(_1_0i, _1_0i), _1_0i * _1_0i + _1_0i);
            assert_eq!(_1_0i * _1_0i + _1_0i, _1_0i.mul_add(_1_0i, _1_0i));

            let mut x = _1_0i;
            x.mul_add_assign(_1_0i, _1_0i);
            assert_eq!(x, _1_0i * _1_0i + _1_0i);

            for &a in &all_consts {
                for &b in &all_consts {
                    for &c in &all_consts {
                        let abc = a * b + c;
                        assert_eq!(a.mul_add(b, c), abc);
                        let mut x = a;
                        x.mul_add_assign(b, c);
                        assert_eq!(x, abc);
                    }
                }
            }
        }

        //#[test]
        pub fn test_mul_add() {
            use num_complex::Complex;
            const _0_0i: Complex<i32> = Complex { re: 0, im: 0 };
            const _1_0i: Complex<i32> = Complex { re: 1, im: 0 };
            const _1_1i: Complex<i32> = Complex { re: 1, im: 1 };
            const _0_1i: Complex<i32> = Complex { re: 0, im: 1 };
            const _neg1_1i: Complex<i32> = Complex { re: -1, im: 1 };
            const all_consts: [Complex<i32>; 5] = [_0_0i, _1_0i, _1_1i, _0_1i, _neg1_1i];

            assert_eq!(_1_0i.mul_add(_1_0i, _0_0i), _1_0i * _1_0i + _0_0i);
            assert_eq!(_1_0i * _1_0i + _0_0i, _1_0i.mul_add(_1_0i, _0_0i));
            assert_eq!(_0_1i.mul_add(_0_1i, _0_1i), _neg1_1i);
            assert_eq!(_1_0i.mul_add(_1_0i, _1_0i), _1_0i * _1_0i + _1_0i);
            assert_eq!(_1_0i * _1_0i + _1_0i, _1_0i.mul_add(_1_0i, _1_0i));

            let mut x = _1_0i;
            x.mul_add_assign(_1_0i, _1_0i);
            assert_eq!(x, _1_0i * _1_0i + _1_0i);

            for &a in &all_consts {
                for &b in &all_consts {
                    for &c in &all_consts {
                        let abc = a * b + c;
                        assert_eq!(a.mul_add(b, c), abc);
                        let mut x = a;
                        x.mul_add_assign(b, c);
                        assert_eq!(x, abc);
                    }
                }
            }
        }

        //#[test]
        pub fn test_div() {
            test_op!(_neg1_1i / _0_1i, _1_1i);
            for &c in all_consts.iter() {
                if c != Zero::zero() {
                    test_op!(c / c, _1_0i);
                }
            }
        }

        //#[test]
        pub fn test_rem() {
            test_op!(_neg1_1i % _0_1i, _0_0i);
            test_op!(_4_2i % _0_1i, _0_0i);
            test_op!(_05_05i % _0_1i, _05_05i);
            test_op!(_05_05i % _1_1i, _05_05i);
            assert_eq!((_4_2i + _05_05i) % _0_1i, _05_05i);
            assert_eq!((_4_2i + _05_05i) % _1_1i, _05_05i);
        }

        //#[test]
        pub fn test_neg() {
            assert_eq!(-_1_0i + _0_1i, _neg1_1i);
            assert_eq!((-_0_1i) * _0_1i, _1_0i);
            for &c in all_consts.iter() {
                assert_eq!(-(-c), c);
            }
        }
    }

    pub mod real_arithmetic {
        use num_complex::Complex;
        use super::{_4_2i, _neg1_1i};

        //#[test]
        pub fn test_add() {
            test_op!(_4_2i + 0.5, Complex::new(4.5, 2.0));
            assert_eq!(0.5 + _4_2i, Complex::new(4.5, 2.0));
        }

        //#[test]
        pub fn test_sub() {
            test_op!(_4_2i - 0.5, Complex::new(3.5, 2.0));
            assert_eq!(0.5 - _4_2i, Complex::new(-3.5, -2.0));
        }

        //#[test]
        pub fn test_mul() {
            assert_eq!(_4_2i * 0.5, Complex::new(2.0, 1.0));
            assert_eq!(0.5 * _4_2i, Complex::new(2.0, 1.0));
        }

        //#[test]
        pub fn test_div() {
            assert_eq!(_4_2i / 0.5, Complex::new(8.0, 4.0));
            assert_eq!(0.5 / _4_2i, Complex::new(0.1, -0.05));
        }

        //#[test]
        pub fn test_rem() {
            assert_eq!(_4_2i % 2.0, Complex::new(0.0, 0.0));
            assert_eq!(_4_2i % 3.0, Complex::new(1.0, 2.0));
            assert_eq!(3.0 % _4_2i, Complex::new(3.0, 0.0));
            assert_eq!(_neg1_1i % 2.0, _neg1_1i);
            assert_eq!(-_4_2i % 3.0, Complex::new(-1.0, -2.0));
        }

        //#[test]
        pub fn test_div_rem_gaussian() {
            // These would overflow with `norm_sqr` division.
            let max = Complex::new(255u8, 255u8);
            assert_eq!(max / 200, Complex::new(1, 1));
            assert_eq!(max % 200, Complex::new(55, 55));
        }
    }

    //#[test]
    pub fn test_to_string() {
        fn test(c: Complex64, s: String) {
            assert_eq!(c.to_string(), s);
        }
        test(_0_0i, "0+0i".to_string());
        test(_1_0i, "1+0i".to_string());
        test(_0_1i, "0+1i".to_string());
        test(_1_1i, "1+1i".to_string());
        test(_neg1_1i, "-1+1i".to_string());
        test(-_neg1_1i, "1-1i".to_string());
        test(_05_05i, "0.5+0.5i".to_string());
    }

    //#[test]
    pub fn test_string_formatting() {
        let a = Complex::new(1.23456, 123.456);
        assert_eq!(format!("{}", a), "1.23456+123.456i");
        assert_eq!(format!("{:.2}", a), "1.23+123.46i");
        assert_eq!(format!("{:.2e}", a), "1.23e0+1.23e2i");
        assert_eq!(format!("{:+.2E}", a), "+1.23E0+1.23E2i");
        #[cfg(feature = "std")]
        assert_eq!(format!("{:+20.2E}", a), "     +1.23E0+1.23E2i");

        let b = Complex::new(0x80, 0xff);
        assert_eq!(format!("{:X}", b), "80+FFi");
        assert_eq!(format!("{:#x}", b), "0x80+0xffi");
        assert_eq!(format!("{:+#b}", b), "+0b10000000+0b11111111i");
        assert_eq!(format!("{:+#o}", b), "+0o200+0o377i");
        #[cfg(feature = "std")]
        assert_eq!(format!("{:+#16o}", b), "   +0o200+0o377i");

        let c = Complex::new(-10, -10000);
        assert_eq!(format!("{}", c), "-10-10000i");
        #[cfg(feature = "std")]
        assert_eq!(format!("{:16}", c), "      -10-10000i");
    }

    //#[test]
    pub fn test_hash() {
        let a = Complex::new(0i32, 0i32);
        let b = Complex::new(1i32, 0i32);
        let c = Complex::new(0i32, 1i32);
        assert!(super::hash(&a) != super::hash(&b));
        assert!(super::hash(&b) != super::hash(&c));
        assert!(super::hash(&c) != super::hash(&a));
    }

    //#[test]
    pub fn test_hashset() {
        use std::collections::HashSet;
        let a = Complex::new(0i32, 0i32);
        let b = Complex::new(1i32, 0i32);
        let c = Complex::new(0i32, 1i32);

        let set: HashSet<_> = [a, b, c].iter().cloned().collect();
        assert!(set.contains(&a));
        assert!(set.contains(&b));
        assert!(set.contains(&c));
        assert!(!set.contains(&(a + b + c)));
    }

    //#[test]
    pub fn test_is_nan() {
        assert!(!_1_1i.is_nan());
        let a = Complex::new(f64::NAN, f64::NAN);
        assert!(a.is_nan());
    }

    //#[test]
    pub fn test_is_nan_special_cases() {
        let a = Complex::new(0f64, f64::NAN);
        let b = Complex::new(f64::NAN, 0f64);
        assert!(a.is_nan());
        assert!(b.is_nan());
    }

    //#[test]
    pub fn test_is_infinite() {
        let a = Complex::new(2f64, f64::INFINITY);
        assert!(a.is_infinite());
    }

    //#[test]
    pub fn test_is_finite() {
        assert!(_1_1i.is_finite())
    }

    //#[test]
    pub fn test_is_normal() {
        let a = Complex::new(0f64, f64::NAN);
        let b = Complex::new(2f64, f64::INFINITY);
        assert!(!a.is_normal());
        assert!(!b.is_normal());
        assert!(_1_1i.is_normal());
    }

    //#[test]
    pub fn test_from_str() {
        fn test(z: Complex64, s: &str) {
            assert_eq!(FromStr::from_str(s), Ok(z));
        }
        test(_0_0i, "0 + 0i");
        test(_0_0i, "0+0j");
        test(_0_0i, "0 - 0j");
        test(_0_0i, "0-0i");
        test(_0_0i, "0i + 0");
        test(_0_0i, "0");
        test(_0_0i, "-0");
        test(_0_0i, "0i");
        test(_0_0i, "0j");
        test(_0_0i, "+0j");
        test(_0_0i, "-0i");

        test(_1_0i, "1 + 0i");
        test(_1_0i, "1+0j");
        test(_1_0i, "1 - 0j");
        test(_1_0i, "+1-0i");
        test(_1_0i, "-0j+1");
        test(_1_0i, "1");

        test(_1_1i, "1 + i");
        test(_1_1i, "1+j");
        test(_1_1i, "1 + 1j");
        test(_1_1i, "1+1i");
        test(_1_1i, "i + 1");
        test(_1_1i, "1i+1");
        test(_1_1i, "+j+1");

        test(_0_1i, "0 + i");
        test(_0_1i, "0+j");
        test(_0_1i, "-0 + j");
        test(_0_1i, "-0+i");
        test(_0_1i, "0 + 1i");
        test(_0_1i, "0+1j");
        test(_0_1i, "-0 + 1j");
        test(_0_1i, "-0+1i");
        test(_0_1i, "j + 0");
        test(_0_1i, "i");
        test(_0_1i, "j");
        test(_0_1i, "1j");

        test(_neg1_1i, "-1 + i");
        test(_neg1_1i, "-1+j");
        test(_neg1_1i, "-1 + 1j");
        test(_neg1_1i, "-1+1i");
        test(_neg1_1i, "1i-1");
        test(_neg1_1i, "j + -1");

        test(_05_05i, "0.5 + 0.5i");
        test(_05_05i, "0.5+0.5j");
        test(_05_05i, "5e-1+0.5j");
        test(_05_05i, "5E-1 + 0.5j");
        test(_05_05i, "5E-1i + 0.5");
        test(_05_05i, "0.05e+1j + 50E-2");
    }

    //#[test]
    pub fn test_from_str_radix() {
        fn test(z: Complex64, s: &str, radix: u32) {
            let res: Result<Complex64, <Complex64 as Num>::FromStrRadixErr> =
                Num::from_str_radix(s, radix);
            assert_eq!(res.unwrap(), z)
        }
        test(_4_2i, "4+2i", 10);
        test(Complex::new(15.0, 32.0), "F+20i", 16);
        test(Complex::new(15.0, 32.0), "1111+100000i", 2);
        test(Complex::new(-15.0, -32.0), "-F-20i", 16);
        test(Complex::new(-15.0, -32.0), "-1111-100000i", 2);
    }

    //#[test]
    pub fn test_from_str_fail() {
        fn test(s: &str) {
            let complex: Result<Complex64, _> = FromStr::from_str(s);
            assert!(
                complex.is_err(),
                "complex {:?} -> {:?} should be an error",
                s,
                complex
            );
        }
        test("foo");
        test("6E");
        test("0 + 2.718");
        test("1 - -2i");
        test("314e-2ij");
        test("4.3j - i");
        test("1i - 2i");
        test("+ 1 - 3.0i");
    }

    //#[test]
    pub fn test_sum() {
        let v = vec![_0_1i, _1_0i];
        assert_eq!(v.iter().sum::<Complex64>(), _1_1i);
        assert_eq!(v.into_iter().sum::<Complex64>(), _1_1i);
    }

    //#[test]
    pub fn test_prod() {
        let v = vec![_0_1i, _1_0i];
        assert_eq!(v.iter().product::<Complex64>(), _0_1i);
        assert_eq!(v.into_iter().product::<Complex64>(), _0_1i);
    }

    //#[test]
    pub fn test_zero() {
        let zero = Complex64::zero();
        assert!(zero.is_zero());

        let mut c = Complex::new(1.23, 4.56);
        assert!(!c.is_zero());
        assert_eq!(&c + &zero, c);

        c.set_zero();
        assert!(c.is_zero());
    }

    //#[test]
    pub fn test_one() {
        let one = Complex64::one();
        assert!(one.is_one());

        let mut c = Complex::new(1.23, 4.56);
        assert!(!c.is_one());
        assert_eq!(&c * &one, c);

        c.set_one();
        assert!(c.is_one());
    }
}
