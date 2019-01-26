# fastfloat

Inspired and based on @bluss's fast-floats:
https://github.com/bluss/fast-floats

I wrote this to speed up my numeric projects, but it should be useful
for most scientific computing applications.

The API is unstable and subject to change.  A nightly rustc is
required. Any feedback, bug reports, and pull requests are very
welcome.

## Example
```rust
use fastfloat::*;

fn main() {
    let mut v:Vec<F32> = Vec::new();

    for i in 0..1000 {
        v.push(fa(i as f32) * 1e-2);
    }

    let s:F32 = v.iter().map(|x| x.fastexp()).sum();
    
    println!("Sum: {}, Avg: {}", s, s / 1e3);
}
```


## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


