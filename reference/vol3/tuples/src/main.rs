fn main() {
    let type_annotated_tuple: (i64, f32, u8) = (500, 6.4, 1);
    let tuple = (500, 6.4, 1); // first defaults to type i32, second to f64, third to i32

    let (x, y, z) = tuple;

    let five_hundred = tuple.0;

    let six_point_four = tuple.1;

    let one = tuple.2;

    println!("The value of y is {y}");
    println!("The value of one is {one}");

    let mut x: (i32, i32) = (1, 2);
    println!("The value of x before mutation is ({}, {})", x.0, x.1);
    x.0 = 0;
    x.1 += 5;
    println!("The value of x after mutation is ({}, {})", x.0, x.1);
}
