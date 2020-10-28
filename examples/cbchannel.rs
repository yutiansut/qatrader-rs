use std::thread;
use std::time::Duration;
use crossbeam_channel::{unbounded,bounded};


fn main() {
    let (s, r) = bounded(0);
    let (ss, rr) = unbounded();
    let r1 = r.clone();
    let r2 = r.clone();
    thread::spawn(move || {
        let mut flag = true;
        loop {
            if let Ok(fl) = rr.try_recv() {
                flag = fl;
            }
            if flag {
                let x = r1.recv().unwrap();
                println!("r1 {}", x);
            }
        }
    });

    thread::spawn(move || {
        loop {
            let x = r2.recv().unwrap();
            println!("     r2 {}", x);
        }
    });

    for i in 1..100 {
        let x = s.send(i);
        if i == 5 {
            ss.send(false);
        }
        if i == 15 {
            ss.send(true);
        }
        if i == 55 {
            ss.send(false);
        }
    }
    // assert_eq!(r.recv(), Ok(1));
}