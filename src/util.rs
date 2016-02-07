
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn ms_timer(ms: u32) -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms as u64));
        tx.send(()).unwrap();
    });
    rx
}

