use crate::OneAtATime;

pub static ONE_AT_A_TIME_GLOBAL_LOCK: OneAtATime = OneAtATime::new();

pub fn one_at_a_time<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    ONE_AT_A_TIME_GLOBAL_LOCK.call(f)
}
