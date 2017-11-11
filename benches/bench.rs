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
fn thousends_sep_int(b: &mut Bencher) {
    b.iter(|| 1234567.thousends_sep());
}

#[bench]
fn thousends_sep_f64(b: &mut Bencher) {
    b.iter(|| 1234567.12345.thousends_sep());
}

#[bench]
fn thousends_sep_f32(b: &mut Bencher) {
    b.iter(|| 1234567.12345f32.thousends_sep());
}
