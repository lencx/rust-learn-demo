fn main() {
    word();
    str_slice();
    str_slice2();

    let a = String::from("hello len");
    let firstword = first_word2(&a[..]);
    // a.clear();
    println!("first word: {}", firstword);

    arr_slice();
}

fn word() {
    println!("**********************");
    let mut s = String::from("hello len");
    let word = first_word(&s);
    println!("{}", word); // 5
    // This empties the String, making it equal to "".
    s.clear();
    println!("{}", s); // ""
    println!("{}", word); // 5
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        println!("i: {}; item: {}", i, &item);
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn str_slice() {
    println!("**********************");
    let _s2 = String::from("hello world");
    let hello = &_s2[0..5];
    let world = &_s2[6..11];
    println!("{}; {}", hello, world); // hello; world
}

fn str_slice2() {
    println!("**********************");
    let s = String::from("hello, len");
    let _a = &s[0..3];
    println!("{}", _a); // hel
    let _a = &s[..2];
    println!("{}", _a); // he
    let _b = &s[2..s.len()];
    println!("{}", _b); // lo, len
    let _b = &s[..];
    println!("{}", _b); // hello, len
}


fn first_word2(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn arr_slice() {
    println!("**********************");
    let a = [1, 4, 6, 8, 9];
    let slice = &a[1..3];
    for (_, &item) in slice.iter().enumerate() {
        println!("{}", &item);
    }
}