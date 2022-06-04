use ::parking_lot::ReentrantMutexGuard;

#[doc(hidden)]
pub type OneAtATimeGuard<'a> = ReentrantMutexGuard<'a, ()>;
