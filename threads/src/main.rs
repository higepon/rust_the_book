use std::sync::mpsc;
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

    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hige");
            tx.send(val).unwrap();
            tx.send(String::from("hage")).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got {}", received);
    }
    {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
