use std::sync::atomic::{ AtomicBool, AtomicI64, Ordering };
use std::time::{ SystemTime, UNIX_EPOCH };
use std::thread;
use std::time::Duration;

static LAST_ACTIVITY: AtomicI64 = AtomicI64::new(0);
static CHECKING_THREAD: AtomicBool = AtomicBool::new(false);

fn get_current_timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn events() {
    LAST_ACTIVITY.store(get_current_timestamp(), Ordering::SeqCst);

    if !CHECKING_THREAD.load(Ordering::SeqCst) {
        CHECKING_THREAD.store(true, Ordering::SeqCst);
        thread::spawn(|| {
            loop {
                let last: i64 = LAST_ACTIVITY.load(Ordering::SeqCst);
                let current: i64 = get_current_timestamp();

                // Check if no one is watching the stream
                if current - last > 10 {
                    idle();
                    break;
                }

                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    println!("Someone is watching the stream");
}

fn idle() {
    println!("No one is watching the stream");
    CHECKING_THREAD.store(false, Ordering::SeqCst);
}
