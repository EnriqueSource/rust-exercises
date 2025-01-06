fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let tup1: (i8, i32, f64) = (-128, 371, 8.3);

    let minus_one_hundred_twenty_eight = tup1.0;
    let three_hundred_seventy_one = tup1.1;
    let eight_point_three = tup1.2;

    println!("The three values of tup1 are: {minus_one_hundred_twenty_eight}, {three_hundred_seventy_one}, and {eight_point_three}");
}
