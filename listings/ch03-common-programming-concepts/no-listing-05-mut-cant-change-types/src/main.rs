fn main() {
    // ANCHOR: here
    let mut spaces = "   ";     // mutable variable
    println!("Before mutating - spaces {spaces}");

    spaces = spaces.len();
    println!("After mutating - spaces {spaces}");
    // ANCHOR_END: here
}
