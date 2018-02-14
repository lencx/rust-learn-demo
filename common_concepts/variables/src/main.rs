fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /* constants */
    const MAX_NUMBER: u32 = 999_999;
    println!("The value of MAX_NUMBER is: {}", MAX_NUMBER);


    /* shadowing */
    let a = 4;
    let a = a + 2;
    let a = a * 3;
    println!("The value of a is: {}", a);

    /* diff (let & mut) */
    // let
    let let_space = "   ";
    let let_space = let_space.len();
    println!("let_space lenght: {}", let_space);
    // mut
    /* let mut mut_space = "   ";
    mut_space = mut_space.len();
    println!("mut_space lenght: {}", mut_space); */
}
