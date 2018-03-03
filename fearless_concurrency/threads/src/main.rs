use std::thread;
use std::time::Duration;

fn main() {
    create_a_new_thread();
    move_closures();
}

fn create_a_new_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn move_closures() {
    println!("---------------------");
    let v = vec![1, 4, 7];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v);
    // println!("v: {:?}", v);
    handle.join().unwrap();
}