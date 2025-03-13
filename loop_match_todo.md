# TODO

* [x] errors instead of ICE on incorrect usage
* [ ] deny drop for `#[const_continue]`
* [x] integer patterns
* [x] `_` and `Foo | Bar` patterns
* [ ] handle in the `let mut` checker (likely needs handling drop trees for StorageDead)
* [ ] `lint_level`?
* [ ] test if nested `#[loop_match]` with `#[const_continue]` operating on outer loop works
* [x] deny attributes on the wrong items
    * [ ] add test
