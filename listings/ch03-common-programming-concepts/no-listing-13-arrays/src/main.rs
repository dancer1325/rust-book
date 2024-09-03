fn main() {
    let a = [1, 2, 3, 4, 5];

    // array has fixed length
    //a[7] = 10;        // ERROR got

    // declare an array
    // [arrayType; numberOfArrayElements]
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize an array / SAME value / ALL elements
    let c = [3; 5];
    println!("{:?}", c);        // Check ALL the values are the SAME
}
