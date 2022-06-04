use ::parking_lot::ReentrantMutex;
use ::std::mem::drop;

use crate::OneAtATimeGuard;

/// Creates a new lock for serialising (or throttling) calls to
/// functions given.
///
/// By creating instances it allows you to use a different lock to the
/// global functions provided.
pub struct OneAtATime {
    inner_lock: ReentrantMutex<()>,
}

impl OneAtATime {
    /// Basic constructor.
    pub const fn new() -> Self {
        Self {
            inner_lock: ReentrantMutex::new(()),
        }
    }

    /// Runs the function given, and returns the result.
    ///
    /// The functionality of this is identical to the function
    /// `one_at_a_time`, with one difference. That function uses a
    /// global lock shared by all calls to it. Whilst calling it from
    /// here, will lock functions using a different lock within `Self`.
    pub fn call<R, F>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let guard = self.lock();
        let result = f();
        drop(guard);

        result
    }

    #[doc(hidden)]
    pub fn lock(&self) -> OneAtATimeGuard {
        self.inner_lock.lock()
    }
}
