fn main() {
    let x = vec![1, 2, 3];
    let y = x; // ownership was transferred here
    let z = x; // 'x' does not own the data anymore, cannot move 'x' to 'z'

    println!("{}", z[0]);
    println!("{}", y[0]);
    println!("{}", x[0]); // cannot access 'x' data anymore
}
