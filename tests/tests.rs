#![feature(test)]

extern crate pretty_toa;
extern crate test;
use pretty_toa::ThousandsSep;

use std::u64;

use std::i64;
use std::i32;

use std::f32;
use std::f64;



#[test]
fn test_0() {
    assert_eq!("0", 0.thousends_sep());
}

#[test]
fn test_1() {
    assert_eq!("1", 1.thousends_sep());
}


#[test]
fn test_12() {
    assert_eq!("12", 12.thousends_sep());
}

#[test]
fn test_123() {
    assert_eq!("123", 123.thousends_sep());
}

#[test]
fn test_1234() {
    assert_eq!("1,234", 1234.thousends_sep());
}

#[test]
fn test_minus_123() {
    assert_eq!("-123", (-123).thousends_sep());
}

#[test]
fn test_minus_1234() {
    assert_eq!("-1,234", (-1234).thousends_sep());
}

#[test]
fn test_max_i32() {
    assert_eq!("2,147,483,647", i32::MAX.thousends_sep());
}

#[test]
fn test_min_i32() {
    assert_eq!("-2,147,483,648", i32::MIN.thousends_sep());
}

#[test]
fn test_max_i64() {
    assert_eq!("9,223,372,036,854,775,807", i64::MAX.thousends_sep());
}

#[test]
fn test_max_u64() {
    assert_eq!("18,446,744,073,709,551,615", u64::MAX.thousends_sep());
}

//
// Floating point tests
//
#[test]
fn test_float_big() {
    assert_eq!(
        "123,000,000,000,000.0",
        123_000_000_000_000f64.thousends_sep()
    );
}
#[test]
fn test_float_nan() {
    assert_eq!("NaN", std::f32::NAN.thousends_sep());
    assert_eq!("NaN", std::f64::NAN.thousends_sep());
}

#[test]
fn test_float_inf() {
    assert_eq!("inf", std::f32::INFINITY.thousends_sep());
    assert_eq!("inf", std::f64::INFINITY.thousends_sep());
}

#[test]
fn test_float_max64() {
    assert_eq!(
        "9,007,199,254,740,992.0",
        9007199254740992f64.thousends_sep()
    );
}

#[test]
fn test_float_0() {
    assert_eq!("0.0", 0.0f32.thousends_sep());
    assert_eq!("0.0", 0.0.thousends_sep());
}

#[test]
fn test_float_1_2() {
    assert_eq!("1.2", 1.2.thousends_sep());
}


#[test]
fn test_float_12() {
    assert_eq!("12.0", 12.0.thousends_sep());
}

#[test]
fn test_float_123() {
    assert_eq!("123.0", 123.0.thousends_sep());
}

#[test]
fn test_float_1234_12() {
    assert_eq!("1,234.12", 1234.12.thousends_sep());
}

#[test]
fn test_float_minus_123_12() {
    assert_eq!("-123.12", (-123.12).thousends_sep());
}


#[test]
fn test_float_minus_1234_0() {
    assert_eq!("-1,234.0", (-1234.0).thousends_sep());
}

#[test]
fn test_small_float() {
    assert_eq!("1.2344e-7", 1234.4e-10.thousends_sep());
}

#[test]
fn test_epsilon_32() {
    assert_eq!("1.1920929e-7", f32::EPSILON.thousends_sep());
}

#[test]
fn test_epsilon_64() {
    assert_eq!("2.220446049250313e-16", f64::EPSILON.thousends_sep());
}

#[test]
fn test_minf64_pos() {
    assert_eq!(
        "2.2250738585072014e-308",
        std::f64::MIN_POSITIVE.thousends_sep()
    );
}

#[test]
fn test_minf32_pos() {
    assert_eq!("1.1754944e-38", std::f32::MIN_POSITIVE.thousends_sep());
}
