fn main() {
    let x: (i32, f64, u8) = (100, 8.9, 1);
    let a = x.0;
    let b = x.1;
    let c = x.2;

    println!("The value of x is ({}, {}, {})", a, b, c);
}
