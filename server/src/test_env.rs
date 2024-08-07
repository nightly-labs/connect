use std::sync::atomic::{AtomicBool, Ordering};

static IS_TEST_ENV: AtomicBool = AtomicBool::new(false);

pub fn is_test_env() -> bool {
    IS_TEST_ENV.load(Ordering::Relaxed)
}

#[cfg(test)]
pub mod test_detection {
    use super::IS_TEST_ENV;
    use std::sync::{atomic::Ordering, Once};

    static INIT: Once = Once::new();

    pub fn setup() {
        INIT.call_once(|| {
            IS_TEST_ENV.store(true, Ordering::Relaxed);
        });
    }
}
