use std::thread;
use std::time::Duration;
use crossbeam_channel::unbounded;


fn main() {
    let (s, r) = unbounded();

    thread::spawn(move || {
        let _ = s.send(1);
        thread::sleep(Duration::from_secs(1));
        let _ = s.send(2);
    });

    assert_eq!(r.recv(), Ok(1)); // Received immediately.
    assert_eq!(r.recv(), Ok(2)); // Received after 1 second.
}