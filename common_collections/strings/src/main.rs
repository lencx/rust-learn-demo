fn main() {
    create_str();
    update_str();
    concat_str();
    index_into_str();
    slice_str();
}

fn create_str() {
    let s = String::new();
    let init_cont = "initial contents".to_string();
    println!("s: {}; init_cont: {}", s, init_cont);
}

fn update_str() {
    let init_cont = "Hi";
    let mut new_cont = init_cont.to_string();
    let s = 'c';
    let s1 = "x !!!";
    new_cont.push_str(", Len");
    new_cont.push(s);
    new_cont.push_str(&s1);
    println!("init_cont: {}", init_cont);
    println!("new_cont: {};", new_cont);
}

fn concat_str() {
    println!("-------------------------");
    let hi = String::from("Hi, ");
    let name = "len";
    let s = hi + &name;
    println!("{}", s);
    // println!("hi: {}", hi); // value used here after move
    println!("name: {}", name);

    println!("-------------------------");
    let year = "2018";
    let month = "Feb";
    let day = "18";

    let date = format!("{} {}/{}", day, month, year);

    println!("date: {}", date);
}

fn index_into_str() {
    println!("-------------------------");
    let s = String::from("Hello");
    // the type `std::string::String` cannot be indexed by `{integer}`
    // let h = s[0];
    println!("s length: {}", s.len());
    println!("nihao: {}", String::from("你好").len()); // 6
}

fn slice_str() {
    println!("-------------------------");
    let nihao = String::from("你好。");
    let s = &nihao[3..6];
    println!("s: {}", s);

    for i in nihao.chars() {
        println!("nihao-item: {}", i);
    }
    for i in nihao.bytes() {
        println!("nihao-item: {}", i);
    }
}