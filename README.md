# pretty_toa

Fast convert numbers to strings with thousends separator.

It is very fast because it uses itoa/dtoa and stack allocated strings.


## Usage Example
```rust
extern crate pretty_toa;
use pretty_toa::ThousandsSep;

fn main() {
    let num = 123_4567;
    println!("{} => {}", num, num.thousends_sep());

    let num  = 123_4567.1234567;
    println!("{} => {}", num, num.thousends_sep());
}
```
