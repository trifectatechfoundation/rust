# TODO

* [x] errors instead of ICE on incorrect usage
* [x] integer patterns
* [x] `_` and `Foo | Bar` patterns
* [x] handle in the `let mut` checker (likely needs handling drop trees for StorageDead)
* [x] `lint_level`?
* [x] test if nested `#[loop_match]` with `#[const_continue]` operating on outer loop works
* [x] deny attributes on the wrong items
    * [x] add test
* [x] fix crash for `match self.bit_reader.bits(2) {}` in zlib-rs
