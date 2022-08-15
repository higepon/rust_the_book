use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} in spawned");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} in main");
        thread::sleep(Duration::from_millis(1));
    }

    {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("vector {:?}", v);
        });
        handle.join().unwrap();
    }
}
