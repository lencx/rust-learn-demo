fn main() {
    let s1 = String::from("Hello");
    let len = calc_len(&s1);
    println!("The length of '{}' is '{}'", s1, len);
    println!("s1: {}", s1);

    let mut s2 = String::from("Hi");
    change_str(&mut s2);

    data_race();

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}


fn calc_len(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(", Len!!!");
    println!("{}", s)
}

fn data_race() {
    let mut s = String::from("HELLO!!");

    {
        let _r1 = &mut s;
    }
    {
        let _r2 = &mut s;
    }

    let _r1 = &s;
    let _r2 = &s;
    // let _r3 = &mut s;
    println!("{}, {}", _r1, _r2);
}

// fn dangle() -> &String {
//     let s = String::from("hello, len");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello, len");
    s
}