fn main() {
    // fn_1();
    fn_2();
    fn_3();
    // fn_4();
    demo_1();

    let a: &'static str = "asdasd salkdal s";
    println!("first word: {}", first_word(&a));

    demo_2();
}

// fn fn_1() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r); // `x` does not live long enough
// }

// fn longest(x: String, y: String) -> String {
//     let _x = x.len();
//     let _y = y.len();
//     if _x > _y {
//         x
//     } else if _x == _y {
//         String::from("'Have Not'")
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let _x = x.len();
    let _y = y.len();

    if _x > _y {
        x
    } else if _x == _y {
        "Have Not"
    } else {
        y
    }
}

fn fn_2() {
    let str1 = String::from("acascdsdl");
    {
        let str2 = String::from("klkds");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", result);
    }
}

fn fn_3() {
    let str1 = String::from("asdadd");
    let result;
    {
        let str2 = "sasd";
        result = longest(str1.as_str(), str2);
    }
    println!("The longest string is {}", result);
}

/*
fn fn_4() {
    let result;
    let str1 = String::from("asdadd");
    {
        let str2 = "sasd";
        result = longest(str1.as_str(), str2);
    }
    // str1` does not live long enough
    println!("The longest string is {}", result);
}
*/

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn demo_1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};
    // mportantExcerpt { part: "Call me Ishmael" }
    println!("{:?}", i);
    println!("{:?}", i.level()); // 3
    println!("{:?}", i.announce_and_return_part(String::from("aaa").as_str())); // Attention please: aaa
    
    // println!("{:?}", first_sentence);
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        // println!("i: {}; item: {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


use std::fmt::Display;

fn longest_width_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_2() {
    println!("------------------------");
    let s1 = String::from("asdjk");
    let s2 = String::from("aad");
    let ann = String::from("ann!!!");
    let _s = longest_width_an_announcement(&s1, &s2, ann);
    println!("_s: {:?}", _s);
}