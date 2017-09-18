fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // The value of x is: 5
    x = 6;
    println!("The value of x is: {}", x);
    // The value of x is: 6

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);
    // The value of x is: 12


    // With shadowing the `spaces` variable can change its type
    let spaces = "   ";
    let spaces = spaces.len();

    // A mutable variable can't change it's type
    // ```rust
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // ```
}
