fn main() {
    new_vec();
    update_vec();
    read_el_vec();
    invalid_references();
    for_vec();
    enum_multi_type();
}

fn new_vec() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let _v = vec![1, 3];
    println!("_v: {:?}", _v);
}

fn update_vec() {
    println!("-------------------------");
    {
        let mut v = Vec::new();
        v.push('a');
        v.push('c');
        v.push('b');
        println!("v: {:?}", v);
    }
    // println!("v: {:?}", v); // error
}

fn read_el_vec() {
    println!("-------------------------");
    let v = vec![1, 4, 6, 9];
    let first: &i32 = &v[0];
    let third: Option<&i32> = v.get(2);
    println!("first: {:?}; third: {:?}", first, third);

    // the len is 4 but the index is 8',...
    // let n = &v[8]; // panic
    let n = v.get(8);
    match n {
        Some(n) => println!("n: {:?}", n),
        None => println!("Out of range!!!"),
    }
}

fn invalid_references() {
    println!("-------------------------");
    let mut v = vec!['a', 'b', 'x', 'y'];
    // let _a = &v[0]; // error
    // println!("_a: {:?}", _a);
    v.pop(); // remove `y`
    v.push('o'); // add `o`
    println!("v: {:?}", v);
}

fn for_vec() {
    println!("-------------------------");
    let mut v = vec![5, 13, 17, 20];
    for i in &mut v {
        *i += 10;
        println!("v-item + 10: {:?}", i);
    }
    println!("v: {:?}", v);
}

fn enum_multi_type() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(120),
        SpreadsheetCell::Text(String::from("Hello, world!!!")),
        SpreadsheetCell::Float(3.14),
    ];

    println!("row: {:?}", row);
}