fn main() {
    let tup = (500, 6.4, 1);

    // destructure tuple's items -- in -- different variables
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
