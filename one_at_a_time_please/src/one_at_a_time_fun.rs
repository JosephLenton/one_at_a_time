use crate::OneAtATime;

/// A global instance of `OneAtATime`.
///
/// It's used internally, but you can also use it too as a singleton
/// if you wish.
pub static ONE_AT_A_TIME_GLOBAL_LOCK: OneAtATime = OneAtATime::new();

/// Runs the function given, and returns the result.
///
/// However if any other threads are also calling functions using this,
/// then all of the functions will be blocked, and run one at a time.
/// In serial, rather than in parallel.
pub fn one_at_a_time<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    ONE_AT_A_TIME_GLOBAL_LOCK.call(f)
}
