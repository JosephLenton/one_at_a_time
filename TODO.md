
# TODO

 - `OneAtATime` struct which allows you to make your own
  - `const fn new()`
  - `fn call<R>(f: FnOnce() -> R) -> R`
 - `one_at_a_time::call(|| {})`
 - #[one_at_a_time] on functions
