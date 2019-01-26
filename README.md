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
    let v:Vec<F32> = (0..1000).map(|x| (fa(x as f32) * 0.01).fastexp()).collect();
    let s:F32 = v.iter().sum();
    
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


