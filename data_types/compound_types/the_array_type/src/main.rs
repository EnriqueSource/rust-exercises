fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    let second = a[1];
    let third = a[2];

    println!("The second value of a is {second} and the third is {third}");

    let a1 = [3; 5];

    let first = a1[0];
    let second = a1[1];
    let third = a1[2];
    let fourth = a1[3];
    let fifth = a1[4];

    println!("array a1 contains the values {first}, {second}, {third}, {fourth} and {fifth}");
}
