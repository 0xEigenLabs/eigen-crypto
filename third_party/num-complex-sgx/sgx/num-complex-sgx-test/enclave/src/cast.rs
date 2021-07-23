use num_complex::*;
use traits::*;

//#[test]
pub fn test_to_primitive() {
    let a: Complex<u32> = Complex { re: 3, im: 0 };
    assert_eq!(a.to_i32(), Some(3_i32));
    let b: Complex<u32> = Complex { re: 3, im: 1 };
    assert_eq!(b.to_i32(), None);
    let x: Complex<f32> = Complex { re: 1.0, im: 0.1 };
    assert_eq!(x.to_f32(), None);
    let y: Complex<f32> = Complex { re: 1.0, im: 0.0 };
    assert_eq!(y.to_f32(), Some(1.0));
    let z: Complex<f32> = Complex { re: 1.0, im: 0.0 };
    assert_eq!(z.to_i32(), Some(1));
}

//#[test]
pub fn test_from_primitive() {
    let a: Complex<f32> = FromPrimitive::from_i32(2).unwrap();
    assert_eq!(a, Complex { re: 2.0, im: 0.0 });
}

//#[test]
pub fn test_num_cast() {
    let a: Complex<f32> = NumCast::from(2_i32).unwrap();
    assert_eq!(a, Complex { re: 2.0, im: 0.0 });
}

//#[test]
pub fn test_as_primitive() {
    let a: Complex<f32> = Complex { re: 2.0, im: 0.2 };
    let a_: i32 = a.as_();
    assert_eq!(a_, 2_i32);
}
