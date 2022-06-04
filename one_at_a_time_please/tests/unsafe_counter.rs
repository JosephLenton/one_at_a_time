use ::one_at_a_time_please::one_at_a_time;
use ::one_at_a_time_please::OneAtATime;
use ::std::cell::UnsafeCell;
use ::std::sync::Arc;
use ::std::thread::spawn;

#[test]
fn it_should_lock_when_using_unsafe_counter_through_proc_attribute() {
    let mut threads = vec![];

    for _ in 0..10 {
        let thread = spawn(|| {
            for _ in 0..100 {
                increment_global_counter();
            }
        });

        threads.push(thread);
    }

    for thread in threads.into_iter() {
        thread.join().expect("This thread should join fine.");
    }

    assert_eq!(UNSAFE_COUNTER.get_count(), 10 * 100);
}

#[test]
fn it_should_lock_when_using_unsafe_counter_through_lambda() {
    let counter = Arc::new(UnsafeCounter::new());

    let mut threads = vec![];
    for _ in 0..10 {
        let thread_local_counter = counter.clone();
        let thread = spawn(move || {
            for _ in 0..100 {
                one_at_a_time(|| {
                    thread_local_counter.increment();
                });
            }
        });

        threads.push(thread);
    }

    for thread in threads.into_iter() {
        thread.join().expect("This thread should join fine.");
    }

    assert_eq!(counter.get_count(), 10 * 100);
}

#[test]
fn it_should_lock_when_using_unsafe_counter_through_struct() {
    let counter = Arc::new(UnsafeCounter::new());
    let oaat = Arc::new(OneAtATime::new());

    let mut threads = vec![];
    for _ in 0..10 {
        let thread_local_counter = counter.clone();
        let thread_local_oaat = oaat.clone();
        let thread = spawn(move || {
            for _ in 0..100 {
                thread_local_oaat.call(|| {
                    thread_local_counter.increment();
                });
            }
        });

        threads.push(thread);
    }

    for thread in threads.into_iter() {
        thread.join().expect("This thread should join fine.");
    }

    assert_eq!(counter.get_count(), 10 * 100);
}

struct UnsafeCounter {
    counter: UnsafeCell<u32>,
}

impl UnsafeCounter {
    pub const fn new() -> Self {
        Self {
            counter: UnsafeCell::new(0),
        }
    }

    pub fn increment(&self) {
        unsafe {
            *self.counter.get() += 1;
        }
    }

    pub fn get_count(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

unsafe impl Sync for UnsafeCounter {}

static UNSAFE_COUNTER: UnsafeCounter = UnsafeCounter::new();

#[one_at_a_time]
fn increment_global_counter() {
    UNSAFE_COUNTER.increment();
}
