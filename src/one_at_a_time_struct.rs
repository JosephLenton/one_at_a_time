use ::parking_lot::Mutex;
use ::parking_lot::MutexGuard;
use ::std::mem::drop;

pub struct OneAtATime {
    inner_lock: Mutex<()>,
}

impl OneAtATime {
    pub const fn new() -> Self {
        Self {
            inner_lock: Mutex::new(()),
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

    pub fn lock(&self) -> MutexGuard<()> {
        self.inner_lock.lock()
    }
}
