fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // this is also a mutable reference
    *y = 10; // modify x through y
    *z = 15;// modify x through z
    println!("x = {}", x);
}