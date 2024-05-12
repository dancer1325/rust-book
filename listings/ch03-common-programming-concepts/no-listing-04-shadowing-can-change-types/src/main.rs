fn main() {
    // ANCHOR: here
    let spaces = "   ";     // string
    println!("Before shadowing - spaces {spaces}");

    let spaces = spaces.len();      // number
    // ANCHOR_END: here
    println!("After shadowing - spaces {spaces}");
}
