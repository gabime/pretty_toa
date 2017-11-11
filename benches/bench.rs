//
// Copyright(c) 2017 Gabi Melman.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)
//

#![feature(test)]

extern crate pretty_toa;
extern crate test;
use test::Bencher;
use pretty_toa::ThousandsSep;

#[bench]
fn thousands_sep_int(b: &mut Bencher) {
    b.iter(|| 1234567.thousands_sep());
}

#[bench]
fn thousands_sep_f64(b: &mut Bencher) {
    b.iter(|| 1234567.12345.thousands_sep());
}

#[bench]
fn thousands_sep_f32(b: &mut Bencher) {
    b.iter(|| 1234567.12345f32.thousands_sep());
}
