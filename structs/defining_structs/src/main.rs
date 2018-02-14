
struct User {
    username: String,
    email: String,
    age: u8,
    sign_in_count: u64,
    active: bool
}

// struct New_User {
//     username: &str,
//     email: &str
// }

fn main() {
    user1();
    mut_user1();

    let user2 = build_user(String::from("len"), String::from("cxin1314@gmail.com"), 28);
    println!("username: {}; email: {}", user2.username, user2.email);

    println!("**************************");
    user3();
    new_structs();

    let red = Color(255, 0, 0);

    println!("**************************");
    // let user4 = New_User {
    //     email: "okasj@example.com",
    //     username: "username43312"
    // };
    
}

fn user1() {
    let user1 = User {
        username: String::from("Lencx"),
        age: 24,
        email: String::from("cxin1314@gmail.com"),
        active: true,
        sign_in_count: 0
    };
    println!("{}", user1.age); // 24
}

fn mut_user1() {
    let mut user1 = User {
        username: String::from("Lencx"),
        age: 24,
        email: String::from("cxin1314@gmail.com"),
        active: true,
        sign_in_count: 0
    };
    user1.age = 25;
    println!("{}", user1.age); // 25
}


fn build_user(username: String, email: String, age: u8) -> User {
    User {
        username,
        email,
        age: age,
        active: true,
        sign_in_count: 4
    }
}

fn user3() {
    let user1 = User {
        username: String::from("Lencx"),
        age: 24,
        email: String::from("cxin1314@gmail.com"),
        active: true,
        sign_in_count: 0
    };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername123"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
        age: 21
    };
    println!("user2: [username: {}, active: {}]", user2.username, user2.active);
}

fn new_structs() {
    let user1 = User {
        username: String::from("Lencx"),
        age: 24,
        email: String::from("cxin1314@gmail.com"),
        active: true,
        sign_in_count: 0
    };
    let user2 = User {
        email: String::from("another2@example.com"),
        username: String::from("anotherusername5467"),
        ..user1
    };
    println!("new structs: [username: {}, active: {}, age: {}]", user2.username, user2.active, user2.age);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);