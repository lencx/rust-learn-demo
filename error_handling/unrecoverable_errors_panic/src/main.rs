fn main() {
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:4
    // panic!("crash and burn");

    panic_backtrace();
}


fn panic_backtrace() {
    let v = vec![1, 4, 6];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 20'
    v[20];
}