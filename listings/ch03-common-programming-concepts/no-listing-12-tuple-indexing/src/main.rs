fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // tupleVariable.index / starts by 0
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of the tuple's items: first is: {five_hundred}, second is {six_point_four} and last one is {one}");

    // "unit" tuple
    // == tuple / NO values
    let unit_tuple = ();
    //println!("unit_tuple {unit_tuple}");  // ERROR, because it's NOT implemented
}
