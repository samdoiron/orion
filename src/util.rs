
use std::sync::mpsc;
use std::thread;

pub fn ms_timer(ms: u32) -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep_ms(ms);
        tx.send(()).unwrap();
    });
    rx
}

