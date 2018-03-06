extern crate what_is_oo;
use what_is_oo::AveragedCollection;

fn main() {
    let mut a = AveragedCollection::new();
    a.add(4);
    a.add(7);
    a.add(6);
    a.add(9);
    // a: AveragedCollection { list: [4, 7, 6, 9], average: 6.5 }
    println!("a: {:?}", a);
    a.remove();
    // a: AveragedCollection { list: [4, 7, 6], average: 5.666666666666667 }
    println!("a: {:?}", a);
    // 5.666666666666667
    println!("average: {}", a.average());
}