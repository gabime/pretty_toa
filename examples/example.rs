//
// Copyright(c) 2017 Gabi Melman.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)
//

extern crate pretty_toa;
use pretty_toa::ThousandsSep;

fn main() {
    let num = 123_4567;
    println!("{} {} => ", num, num.thousends_sep());

    let num = 123_4567.1234567;
    println!("{} => {}", num, num.thousends_sep());
}
