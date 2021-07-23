// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The exponential distribution.

use crate::utils::{ziggurat, Float};
use crate::{ziggurat_tables, Distribution};
use rand::Rng;
use std::{error, fmt};

/// Samples floating-point numbers according to the exponential distribution,
/// with rate parameter `λ = 1`. This is equivalent to `Exp::new(1.0)` or
/// sampling with `-rng.gen::<f64>().ln()`, but faster.
///
/// See `Exp` for the general exponential distribution.
///
/// Implemented via the ZIGNOR variant[^1] of the Ziggurat method. The exact
/// description in the paper was adjusted to use tables for the exponential
/// distribution rather than normal.
///
/// [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to
///       Generate Normal Random Samples*](
///       https://www.doornik.com/research/ziggurat.pdf).
///       Nuffield College, Oxford
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::Exp1;
///
/// let val: f64 = thread_rng().sample(Exp1);
/// println!("{}", val);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Exp1;

impl Distribution<f32> for Exp1 {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        // TODO: use optimal 32-bit implementation
        let x: f64 = self.sample(rng);
        x as f32
    }
}

// This could be done via `-rng.gen::<f64>().ln()` but that is slower.
impl Distribution<f64> for Exp1 {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        #[inline]
        fn pdf(x: f64) -> f64 {
            (-x).exp()
        }
        #[inline]
        fn zero_case<R: Rng + ?Sized>(rng: &mut R, _u: f64) -> f64 {
            ziggurat_tables::ZIG_EXP_R - rng.gen::<f64>().ln()
        }

        ziggurat(
            rng,
            false,
            &ziggurat_tables::ZIG_EXP_X,
            &ziggurat_tables::ZIG_EXP_F,
            pdf,
            zero_case,
        )
    }
}

/// The exponential distribution `Exp(lambda)`.
///
/// This distribution has density function: `f(x) = lambda * exp(-lambda * x)`
/// for `x > 0`.
///
/// Note that [`Exp1`](crate::Exp1) is an optimised implementation for `lambda = 1`.
///
/// # Example
///
/// ```
/// use rand_distr::{Exp, Distribution};
///
/// let exp = Exp::new(2.0).unwrap();
/// let v = exp.sample(&mut rand::thread_rng());
/// println!("{} is from a Exp(2) distribution", v);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Exp<N> {
    /// `lambda` stored as `1/lambda`, since this is what we scale by.
    lambda_inverse: N,
}

/// Error type returned from `Exp::new`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `lambda <= 0` or `nan`.
    LambdaTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::LambdaTooSmall => "lambda is not positive in exponential distribution",
        })
    }
}

impl error::Error for Error {}

impl<N: Float> Exp<N>
where Exp1: Distribution<N>
{
    /// Construct a new `Exp` with the given shape parameter
    /// `lambda`.
    #[inline]
    pub fn new(lambda: N) -> Result<Exp<N>, Error> {
        if !(lambda > N::from(0.0)) {
            return Err(Error::LambdaTooSmall);
        }
        Ok(Exp {
            lambda_inverse: N::from(1.0) / lambda,
        })
    }
}

impl<N: Float> Distribution<N> for Exp<N>
where Exp1: Distribution<N>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> N {
        rng.sample(Exp1) * self.lambda_inverse
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exp() {
        let exp = Exp::new(10.0).unwrap();
        let mut rng = crate::test::rng(221);
        for _ in 0..1000 {
            assert!(exp.sample(&mut rng) >= 0.0);
        }
    }
    #[test]
    #[should_panic]
    fn test_exp_invalid_lambda_zero() {
        Exp::new(0.0).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_exp_invalid_lambda_neg() {
        Exp::new(-10.0).unwrap();
    }

    #[test]
    fn value_stability() {
        fn test_samples<N: Float + core::fmt::Debug, D: Distribution<N>>(
            distr: D, zero: N, expected: &[N],
        ) {
            let mut rng = crate::test::rng(223);
            let mut buf = [zero; 4];
            for x in &mut buf {
                *x = rng.sample(&distr);
            }
            assert_eq!(buf, expected);
        }

        test_samples(Exp1, 0f32, &[1.079617, 1.8325565, 0.04601166, 0.34471703]);
        test_samples(Exp1, 0f64, &[
            1.0796170642388276,
            1.8325565304274,
            0.04601166186842716,
            0.3447170217100157,
        ]);

        test_samples(Exp::new(2.0).unwrap(), 0f32, &[
            0.5398085, 0.91627824, 0.02300583, 0.17235851,
        ]);
        test_samples(Exp::new(1.0).unwrap(), 0f64, &[
            1.0796170642388276,
            1.8325565304274,
            0.04601166186842716,
            0.3447170217100157,
        ]);
    }
}
