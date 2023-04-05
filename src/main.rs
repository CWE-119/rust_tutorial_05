fn main() {
    let x = add(1,1);
    println!("x {}", x);
    let y = add(x, 1);
    println!("y {}", y);
}

fn add(a:i64, b:i64) -> i64{
    a + b
}
