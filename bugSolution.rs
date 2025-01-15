fn main() {
    let mut x = 5;
    { //create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // modify x through y
    }
    { // create a new scope
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}