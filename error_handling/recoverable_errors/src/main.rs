// #[derive(Debug)]
// enum Reslut<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    create_file();
    let _username = read_username();
    let _t = shortcut_propagating_err();

    // cannot use the `?` operator in a function that returns `()`
    // File::open("user.txt")?;
}

fn create_file() {
    let filename = "hello.txt";
    // let f = File::open(&filename).unwrap();
    // let f = File::open(&filename).expect("Failed to open hello.txt");
    let f = File::open(&filename);
    match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            // println!("err: {}", err); // No such file or directory 
            // println!("err.kind(): {:?}", err.kind()); // NotFound
            match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },
        Err(err) => panic!("There was a problem opening the file: {:?}", err)
    };
}

fn read_username() -> Result<String, io::Error> {
    let filename = "user.txt";
    let f = File::open(&filename);
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => {
            println!("{:?}", s); // lencx
            Ok(s)
        },
        Err(e) => Err(e),
    }
}

fn shortcut_propagating_err() -> Result<String, io::Error> {
    let filename = "hi.txt";
    let mut s = String::new();
    // let mut f = File::open(&filename)?;
    // f.read_to_string(&mut s)?;
    // Ok(s)

    File::open(&filename)?.read_to_string(&mut s)?;
    println!("{}", s); // Hi, lencx!!!
    Ok(s)
}