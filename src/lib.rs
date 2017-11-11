//
// Copyright(c) 2017 Gabi Melman.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)
//


//!
//! Fast convert numbers to strings with thousands separator.
//!
//! It is very fast because it uses itoa/dtoa and stack allocated strings.
//!
//! # Example
//!
//! ```
//! extern crate pretty_toa;
//! use pretty_toa::ThousandsSep;
//!
//! fn main() {
//!     let num = 123_4567;
//!     println!("{} => {}", num, num.thousends_sep());
//!
//!     let num  = 123_4567.1234567;
//!     println!("{} => {}", num, num.thousends_sep());
//! }
//! ```

extern crate dtoa;
extern crate itoa;
extern crate num_traits;
use std::str;

fn thousand_sep<T>(num: T) -> String
where
    T: itoa::Integer + num_traits::PrimInt,
{
    // write to a stack buffer
    let mut bytes = [b'\0'; 20];
    let n = itoa::write(&mut bytes[..], num).expect("itoa failed");
    let int_string = &bytes[..n];
    let orig_len = int_string.len();
    let mut rv = Vec::with_capacity(orig_len + orig_len / 3);

    let offset = orig_len % 3;
    let min_comma_pos = if num >= num_traits::zero::<T>() { 0 } else { 1 };
    for (i, digit) in int_string.iter().enumerate() {
        if i > min_comma_pos && (i % 3) == offset {
            rv.push(',' as u8);
        }
        rv.push(*digit as u8);
    }
    unsafe { String::from_utf8_unchecked(rv) }
}


fn thousand_sep_f<T>(num: T) -> String
where
    T: dtoa::Floating + num_traits::Float,
{
    // Handle special floats first
    if num.is_nan() {
        return "NaN".to_string();
    }

    if num.is_infinite() {
        return "inf".to_string();
    }

    // write to a stack buffer
    let mut bytes = [b'\0'; 64];
    let n = dtoa::write(&mut bytes[..], num).expect("dtoa failed write to buffer");
    //let float_string = str::from_utf8(&bytes[..n]).unwrap();
    let float_string = unsafe { str::from_utf8_unchecked(&bytes[..n]) };

    let orig_len = float_string.len();

    let mut rv = String::with_capacity(orig_len + orig_len / 3);

    let decimal_point_index = float_string
        .find('.')
        .expect("decminal point not found in float string");

    let min_comma_pos = if num >= num_traits::zero::<T>() { 0 } else { 1 };
    let offset = decimal_point_index % 3;
    for (i, digit) in float_string[..decimal_point_index].chars().enumerate() {
        if i > min_comma_pos && (i % 3) == offset {
            rv.push(',');
        }
        rv.push(digit);
    }
    // Add the fraction part after the decimal point
    rv.push_str(&float_string[decimal_point_index..]);
    rv
}


pub trait ThousandsSep {
    fn thousends_sep(self) -> String;
}


//
// integers
//

// 8 bits
impl ThousandsSep for i8 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

impl ThousandsSep for u8 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}



// 16 bits
impl ThousandsSep for i16 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

impl ThousandsSep for u16 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

// 32 bits
impl ThousandsSep for i32 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

impl ThousandsSep for u32 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

// 64 bits
impl ThousandsSep for i64 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

impl ThousandsSep for u64 {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

// arch
impl ThousandsSep for isize {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}

impl ThousandsSep for usize {
    fn thousends_sep(self) -> String {
        thousand_sep(self)
    }
}


//
// floating
//

impl ThousandsSep for f32 {
    fn thousends_sep(self) -> String {
        thousand_sep_f(self)
    }
}

impl ThousandsSep for f64 {
    fn thousends_sep(self) -> String {
        thousand_sep_f(self)
    }
}
