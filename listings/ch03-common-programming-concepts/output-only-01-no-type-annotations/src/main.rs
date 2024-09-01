fn main() {
    // MANY types are possible & data type NOT specified -> get an error
    let guess = "42".parse().expect("Not a number!");
}
