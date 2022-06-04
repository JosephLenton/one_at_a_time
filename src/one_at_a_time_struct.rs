use ::parking_lot::ReentrantMutex;
use ::std::mem::drop;

use crate::OneAtATimeGuard;

pub struct OneAtATime {
    inner_lock: ReentrantMutex<()>,
}

impl OneAtATime {
    pub const fn new() -> Self {
        Self {
            inner_lock: ReentrantMutex::new(()),
        }
    }

    pub fn call<R, F>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let guard = self.lock();
        let result = f();
        drop(guard);

        result
    }

    pub fn lock(&self) -> OneAtATimeGuard {
        self.inner_lock.lock()
    }
}
