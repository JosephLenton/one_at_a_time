use ::parking_lot::ReentrantMutexGuard;

pub type OneAtATimeGuard<'a> = ReentrantMutexGuard<'a, ()>;
