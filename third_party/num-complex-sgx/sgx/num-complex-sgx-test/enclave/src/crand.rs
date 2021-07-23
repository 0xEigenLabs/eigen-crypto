use num_complex::*;
use rand::distributions::Standard;
use rand::prelude::*;

fn test_rng() -> SmallRng {
    SmallRng::from_seed([42; 16])
}

//#[test]
pub fn standard_f64() {
    let mut rng = test_rng();
    for _ in 0..100 {
        let c: Complex<f64> = rng.gen();
        assert!(c.re >= 0.0 && c.re < 1.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

//#[test]
pub fn generic_standard_f64() {
    let mut rng = test_rng();
    let dist = ComplexDistribution::new(Standard, Standard);
    for _ in 0..100 {
        let c: Complex<f64> = rng.sample(&dist);
        assert!(c.re >= 0.0 && c.re < 1.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

//#[test]
pub fn generic_uniform_f64() {
    use rand::distributions::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100.0, 0.0);
    let im = Uniform::new(0.0, 100.0);
    let dist = ComplexDistribution::new(re, im);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(&dist);
        assert!(c.re >= -100.0 && c.re < 0.0);
        assert!(c.im >= 0.0 && c.im < 100.0);
    }
}

//#[test]
pub fn generic_mixed_f64() {
    use rand::distributions::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100.0, 0.0);
    let dist = ComplexDistribution::new(re, Standard);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(&dist);
        assert!(c.re >= -100.0 && c.re < 0.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

//#[test]
pub fn generic_uniform_i32() {
    use rand::distributions::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100, 0);
    let im = Uniform::new(0, 100);
    let dist = ComplexDistribution::new(re, im);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(&dist);
        assert!(c.re >= -100 && c.re < 0);
        assert!(c.im >= 0 && c.im < 100);
    }
}
