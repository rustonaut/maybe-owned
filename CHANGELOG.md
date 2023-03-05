
## Change Log

- `pending` (0.4?,1.0?)
  - Added `Cow` from `MaybeOwned` implementation
  - Removed deprecated method (`to_mut`)
  - Extended impl. of `PartialOrd` to allow other
    right hand side types.
  - Feature gates transitive ops implementations
    (and marked them as unstable).
  - Transitive ops implementations now return a
    `MaybeOwend`/`MaybeOwnedMut` as to be more
    consistent with other API's and allow Things
    like `a + b + c`.

- `v0.3.4`:
  - Added `make_owned()` as a `to_mut()` replacement,
    but also available for `MaybeOwnedMut` and more
    clear in it's functionality.
  - Added a `as_mut()` method to `MaybeOwned` which
    return a `Option<&mut T>`
  - Added missing `BorrowMut` implementation
    for `MaybeOwnedMut`


- `v0.3.3`:
  - added `MaybeOwnedMut`

- `v0.3.2`:
  - added transitive `std::ops` implementations

- `v0.3.1`:
  - added `serde` support

