# pretty_toa

Rust crate for converting numbers (integers and floats) to strings with thousands separators.

It is very fast because it uses itoa/dtoa and stack allocated strings.


## Usage Example
```rust
extern crate pretty_toa;
use pretty_toa::ThousandsSep;

fn main() {
    let num = 123_4567;
    println!("{} => {}", num, num.thousands_sep());

    let num  = 123_4567.1234567;
    println!("{} => {}", num, num.thousands_sep());
}
```
