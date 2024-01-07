use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Using join here locks the main thread until the spawn thread finishes
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Using join here will have both threads alternate until the main thread finishes
    // and then hang the ending of the process until the spawn thread can finish
    // handle.join().unwrap();
}
