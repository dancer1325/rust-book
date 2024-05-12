fn main() {
    let x = 5;
    println!("The value of x is: {x}");     // {x}      placeholder
    x = 6;          // ERROR, because it's immutable by default
    println!("The value of x is: {x}");
}
