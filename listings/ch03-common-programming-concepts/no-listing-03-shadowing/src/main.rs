fn main() {
    let x = 5;

    let x = x + 1;      // shadowing

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // Shadowing at global level dependent
    println!("The value of x is: {x}");
}
