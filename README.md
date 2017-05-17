
# maybe-owned &emsp; [![Build Status](https://travis-ci.org/dathinab/maybe-owned.svg?branch=master)](https://travis-ci.org/dathinab/maybe-owned)

**provides a `MaybeOwned<'a,T>` enum with `From<T>` and `From<&'a T>`, different to `Cow` it does not require `ToOwned`**

---

This crate provides a `MaybeOwned<'a,T>` enum. Different to `std::borrow::Cow` it
implements `From<T>` and `From<&'a T>` and does not require a `ToOwned` implementation.
While this can be nice for API's mainly consuming T's not implementing `ToOwned` or implementing 
`ToOwned` through `Clone` it also means it's borrowed version of `String` is 
`&String` and not `&str` making it less performant for cases like `String` or `Vec`.


Documentation can be [viewed on docs.rs](https://docs.rs/maybe-owned).


## Example

The main benefit of `MaybeOwned` over `Cow` is for API design:

```rust

struct RegexRegestry {
    
}
```




## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
