use std::{thread, time::Duration};

pub fn demo() {
    println!("Threads");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    println!("Joined");

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("{}: Here's a vector: {:?}", i, v);
        }
    });
    handle.join().unwrap();
}
