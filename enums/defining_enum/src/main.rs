#[derive(Debug)]
enum Message {
    // Quit,
    Move {x: i32, y: i32},
    Write(String),
    // ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    ip_addr_1();
    ip_addr_2();

    let _write = Message::Write(String::from("Hello, Len"));
    let _move = Message::Move {x: 20, y: -20};
    _write.call();
    _move.call();

    println!("*******************");
    let some_num = Some(8);
    let some_str = Some("a string!!!");
    println!("some_num: {:?}; some_str: {:?}", some_num, some_str);

    // let x: i8 = 12;
    // let y: Option<i8> = Some(8);
    // let sum = x + y;
    // println!("sum: {:?}", sum);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6(String),
}
#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}
fn ip_addr_1() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    println!("{:?}", _four);

    let _home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("_home: {:?}", _home);

    let _edu = IpAddrKind::V6(String::from("::1"));
    println!("_edu: {:?}", _edu);
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_addr_2() {
    let _home = IpAddr2::V4(192, 168, 1, 1);
    let _edu = IpAddr2::V6(String::from("::1"));
    println!("_home: {:?}; _edu: {:?}", _home, _edu);
}