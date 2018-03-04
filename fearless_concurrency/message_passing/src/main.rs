use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    msg_passing_1();
    msg_passing_2();
}

fn msg_passing_1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // value used here after move
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}!", received);
}

fn msg_passing_2() {
    println!("------------------------");
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // tx.send(val).unwrap();
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