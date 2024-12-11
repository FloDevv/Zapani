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
    LAST_ACTIVITY.store(get_current_timestamp(), Ordering::Release);

    if !CHECKING_THREAD.load(Ordering::Acquire) {
        CHECKING_THREAD.store(true, Ordering::Release);
        thread::spawn(|| {
            let mut consecutive_idles: i32 = 0;
            loop {
                let last: i64 = LAST_ACTIVITY.load(Ordering::Acquire);
                let current: i64 = get_current_timestamp();

                if current - last > 10 {
                    consecutive_idles += 1;
                    if consecutive_idles >= 3 {
                        // Wait for 3 consecutive idle checks
                        idle();
                        break;
                    }
                } else {
                    consecutive_idles = 0;
                }

                thread::sleep(Duration::from_secs(10));
            }
        });
    }
}

fn idle() {
    println!("No one is watching the stream");
    CHECKING_THREAD.store(false, Ordering::SeqCst);
}
