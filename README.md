# One At A Time Please

Mark functions as being `#[one_at_a_time]`, and instantly only one
thread can call them at a time. It's quick and dirty synchronisation of
calls to functions.

## What is for?

Lets say you have a number of functions. They will get called from multiple threads.
You only want one of them ever called at a time. When that happens you want the rest to all be blocked.

For example; you have a load of tests. You want to run them in parallel, except for four of them. Which should run in serial.

(Single-threaded recursion is still allowed.)

## How does it work?

Internally there is a big lock. Functions marked with `one_at_a_time` will aquire that big lock. When they have it, then everyone else is blocked.

## API

 - `#[one_at_a_time]` -- An annotation to add to functions. Functions marked with this can only be called on one thread at a time. Otherwise they (politely) wait until it's their turn.
 - `one_at_a_time()` -- A helper function to do this yourself in code. You call it, pass in a lambda, and it will (politely) wait until the lock is free. Then run.
 - `OneAtATime` - All of the other APIs share one giant lock. This is a struct that allows you to make your own lock, and use that instead.

## Examples

### `#[one_at_a_time]`

```Rust
use ::one_at_a_time_please::one_at_a_time;

/// This __cannot__ be called from different threads at the same time.
#[one_at_a_time]
fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

### `one_at_a_time<F>(f: FnOnce() -> R) -> F`

```Rust
use ::one_at_a_time_please::one_at_a_time;

/// This __can__ be called from different threads at the same time ...
fn some_function(a: u32, b: u32) -> u32 {
    /// but this __cannot__ be called from different threads.
    one_at_a_time(move || {
        a + b
    })
}
```

### `struct OneAtATime`

```Rust
use ::one_at_a_time_please::one_at_a_time;
use ::std::sync::Arc;

/// This example is for showing that you can make your own lock,
/// and block on that.
///
/// This is a different lock to what `#[one_at_a_time]` uses.
/// It's behaviour will not overlap with them.
#[one_at_a_time]
fn some_function(a: u32, b: u32) -> u32 {
    let oaat = Arc::new(OneAtATime::new());

    for i in 0 .. 10 {
        let thread_local_oaat = oaat.clone();
        let thread = spawn(move || {
            for _ in 0..10 {
                thread_local_oaat.call(|| {
                    // do stuff here
                });
            }
        });
    }
}
```

### Advanced Example

An example of a (terrible) unsafe counter is provided in tests.
Where it's made safe to call using `#[one_at_a_time]`.

You can find it at [tests/unsafe_counter.rs](./tests/unsafe_counter.rs).
