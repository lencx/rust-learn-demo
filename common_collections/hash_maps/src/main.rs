use std::collections::HashMap;

fn main() {
    create_map();
    hashmap_ownership();
    read_hashmap_val();
    hashmap_update();
}


fn create_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 12);
    println!("scores: {:?}", scores);

    println!("**********************");

    let teams = vec![String::from("Green"), String::from("Yellow")];
    let init_scores = vec![30, 20];
    let _scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("_scores: {:?}", _scores);
}

fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("skyblue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // value used here after move
    // println!("map: {:?}", map);
    // println!("field_name: {:?}", field_name);
}

fn read_hashmap_val() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 15);
    scores.insert(String::from("Blue"), 23);

    let get_red = scores.get(&String::from("Red"));
    println!("Red Scores: {:?}", get_red);

    for (key, val) in &scores {
        println!("key: {}, value: {}", key, val);
    }
}

fn hashmap_update() {
    println!("-------------- 1 ----------------");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);
    scores.insert(String::from("Blue"), 18);
    println!("scores: {:?}", scores);

    println!("--------------- 2 ---------------");
    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(15);
    // scores: {"Blue": 18, "Red": 15}
    println!("scores: {:#?}", scores);

    println!("--------------- 3 ---------------");
    let txt = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("words number: {:?}", map);
}