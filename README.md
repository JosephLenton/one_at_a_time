> One At A Time Please

A library for restricting function calls from across multiple threads,
to only happen one at a time. Forcing your code to run in serial.

**It is for** ... quick and dirty synchronisation, when you just need it.
It is not for well thought out synchronisation.
Specifically I built this for testing. I had about 20 tests, where a couple needed to capture panic information.
They had to do it 'one at a time'. I wrote this to make that easier.

# API

 - `#[one_at_a_time]` -- A function annotation that makes functions run in serial.
 - `one_at_a_time()` -- A function
 - OneAtATime - A struct
# Usage

```Rust
#[one_at_a_time]
fn my_unsafe_function(foo: String, bar: u32) -> f32 {
  // code omitted
}
```
