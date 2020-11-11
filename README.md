Pollard's p-1 Factoring Algorithm
[![Latest Version](https://img.shields.io/crates/v/pollard-p-minus-one.svg)](https://crates.io/crates/pollard-p-minus-one)
[![Documentation](https://docs.rs/pollard-p-minus-one/badge.svg)](https://docs.rs/pollard-p-minus-one)
---------------------------------

A Rust implementation of Pollard's p-1 factoring algorithm. This algorithm can quickly factor an integer `n` if `p` is a `factor` of `n` and `p - 1` is `b-powersmooth`. This means that all prime powers of `p` are less than or equal to `b`.

## Installation

Add it as a dependency in your Cargo.toml file:
```
pollard-p-minus-one = "*"
```

Note that because this crate depends on [ramp](https://crates.io/crates/ramp) which only compiles on nightly, you must use a nightly build to use this crate.

## Example

Pass the integer `n` you want to factor and your guess for `b` to the `factor` function. For example, if `n` is 299, it can be factored as follows:
```
use pollard_p_minus_one::factor;

let n = 299;
let b = 4;
println!("Found factor {}", factor(n, b).unwrap());
```

4 is a good choice for `b` in this case because 229 has a factor `p` of 13, and `p - 1` is 12 which is 4-powersmooth (`2^2 * 3^1 = 12`, all prime powers are less than or equal to 4). If you choose a `b` that's too small or too large, no factors will be found. A `b` of 3 won't work in this case, and while a `b` from 4 to 10 will work, 11 or higher won't work.

Obviously, you can't rely on knowing the factorization of `n` to choose `b`, since the whole point is to find that factorization. If you guess a large value for `b` (like 2^32), this implementation will attempt to factor `n` using increments of 10000 up to your guessed value. This isn't guaranteed to work and will be a little slower than normal, so the closer you can get to the real value of `b` the better.

If the integer you'd like to factor is larger than a `u64`, you can pass it to `factor` using the `ramp::Int::from_str()` function from the [ramp](https://crates.io/crates/ramp) crate: `factor(ramp::Int::from_str("348242219231"), b)`.
